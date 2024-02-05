use axum::async_trait;
use bb8_redis::redis::AsyncCommands;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::redis::RedisDatabase,
    service::task::{
        error::TaskError,
        message::{TaskRequest, TaskResponse, TaskStatus},
        Task, TaskHandler,
    },
};

use super::model::UserSession;

pub struct SessionTaskHandler;

#[async_trait]
impl TaskHandler<RedisDatabase> for SessionTaskHandler {
    async fn handle(redis: &RedisDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("session_create") {
            let payload =
                match TaskRequest::intepret_request_payload::<SessionCreateTask>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return SessionCreateTask::run(redis, task_request, payload).await;
        }
        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionCreateTask {
    pub token: String,
    pub expires_in: u128,
    pub user_id: String,
}

#[async_trait]
impl Task<RedisDatabase, TaskRequest, SessionCreateTask> for SessionCreateTask {
    async fn run(
        db: &RedisDatabase,
        request: TaskRequest,
        param: SessionCreateTask,
    ) -> TaskResponse {
        // todo later when needed verify if the user_id is connected to a real user.

        let mut pool = db.pool.get().await.unwrap();

        //let silhouette_token = Uuid::new_v4().as_simple().to_string();

        let hash: Vec<(String, String)> = pool.hgetall("user-sessions").await.unwrap();
        if let Some(existing_key) =
            hash.iter()
                .find_map(|(k, v)| if v.eq(&param.user_id) { Some(k) } else { None })
        {
            // If an existing session is found, delete it
            pool.hdel::<&str, &str, ()>("user-sessions", existing_key)
                .await
                .unwrap();
        }
        // Set the new session token for the user
        let hset_result = pool
            .hset::<&str, String, String, ()>(
                "user-sessions",
                param.token.clone(),
                param.user_id.clone(),
            )
            .await;

        // if token failed to create
        match hset_result {
            Ok(_) => {
                pool.expire::<&str, i32>("user-sessions", param.expires_in.try_into().unwrap()) // 7 days in seconds
                    .await
                    .unwrap();
            }
            Err(_) => {
                // should not happen...
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::SessionCreationFailed.to_string()],
                );
            }
        }

        return TaskResponse::compose_response(
            request,
            TaskStatus::Completed,
            UserSession::new(&param.token, param.expires_in, &param.user_id),
            Vec::default(),
        );
    }
}
