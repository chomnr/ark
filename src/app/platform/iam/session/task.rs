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
    pub expires_in: i64,
    pub user_id: String,
}

#[async_trait]
impl Task<RedisDatabase, TaskRequest, SessionCreateTask> for SessionCreateTask {
    async fn run(
        db: &RedisDatabase,
        request: TaskRequest,
        param: SessionCreateTask,
    ) -> TaskResponse {
        let mut pool = db.pool.get().await.unwrap();
        let hash: Vec<(String, String)> = pool.hgetall("user-sessions").await.unwrap();
        if let Some(existing_key) = hash.iter().find_map(|(k, v)| {
            if let Ok(session) = serde_json::from_str::<UserSession>(v) {
                if session.user_id.eq(&param.user_id) {
                    return Some(k);
                }
            }
            None
        }) {
            pool.hdel::<&str, &str, ()>("user-sessions", existing_key)
                .await
                .unwrap();
        }
        // Set the new session token for the user
        let _ = pool
            .hset::<&str, String, String, ()>(
                "user-sessions",
                param.token.clone(),
                serde_json::to_string(&param).unwrap(),
            )
            .await;
        return TaskResponse::compose_response(
            request,
            TaskStatus::Completed,
            UserSession::new(&param.token, param.expires_in, &param.user_id),
            Vec::default(),
        );
    }
}
