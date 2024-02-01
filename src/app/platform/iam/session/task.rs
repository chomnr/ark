use axum::async_trait;

use crate::app::{database::{postgres::PostgresDatabase, redis::RedisDatabase}, service::task::{error::TaskError, message::{TaskRequest, TaskResponse}, TaskHandler}};

pub struct SessionTaskHandler;

#[async_trait]
impl TaskHandler<RedisDatabase> for SessionTaskHandler {
    async fn handle(redis: &RedisDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("session_create") {
            todo!()
        }
        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

// redis is used instead of postgres;
// user is cached
// the session is not cached.. (even though it is in the database.)