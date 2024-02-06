use std::{mem, thread::scope};

use axum::async_trait;
use bb8_redis::redis::{AsyncCommands, AsyncIter, RedisError};
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
        //let hash: Vec<(String, String)> = pool.hgetall("user-sessions").await.unwrap();
        //if let Some(existing_key) =
        //    hash.iter()
        //        .find_map(|(k, v)| if v.eq(&param.user_id) { Some(k) } else { None })
        //{
        //    // If an existing session is found, delete it
        //    pool.hdel::<&str, &str, ()>("user-sessions", existing_key)
        //        .await
        //        .unwrap();
        //}
        let pattern = format!("session:*:{}", param.user_id);
        let mut scan_result: AsyncIter<String> = pool.scan_match(&pattern).await.unwrap();
        // need to collect into a vec to go around rust borrowing rules
        let mut sessions_to_invalidate: Vec<String> = Vec::new();
        while let Some(key_result) = scan_result.next_item().await {
            sessions_to_invalidate.push(key_result);
        }
        // drop scan_result after loop so it can be reused.
        mem::drop(scan_result);
        // invalidate any existing session.
        for key in sessions_to_invalidate.iter() {
            let _: () = pool.del(key).await.unwrap();
        }
        // Set the new session token for the user
        //  let hset_result = pool
        //    .hset::<&str, String, String, ()>(
        //        &session_key,
        //        param.token.clone(),
        //        param.user_id.clone(),
        //    )1
        //    .await;
        // session key
        let session_key = format!("session:{}:{}", param.token, param.user_id).to_string();
        let hset_result = pool
            .set::<&str, String, ()>(&session_key, param.user_id.clone())
            .await;
        match hset_result {
            Ok(_) => {
                pool.expire::<&str, i64>(&session_key, param.expires_in) // 7 days in seconds
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
