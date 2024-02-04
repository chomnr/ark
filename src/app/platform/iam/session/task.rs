use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{database::redis::RedisDatabase, service::task::{error::TaskError, message::{TaskRequest, TaskResponse}, Task, TaskHandler}};

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
    token: String,
    expires_in: u128,
    user_id: String
}

#[async_trait]
impl Task<RedisDatabase, TaskRequest, SessionCreateTask> for SessionCreateTask {
    async fn run(
        db: &RedisDatabase,
        request: TaskRequest,
        param: SessionCreateTask,
    ) -> TaskResponse {
        // link session to user_id
        todo!()
    }
}
// task: toke
// expires_in:
// user_id the token will be linked to this id.

// redis is used instead of postgres;
// user is cached
// the session is not cached.. (even though it is in the database.)