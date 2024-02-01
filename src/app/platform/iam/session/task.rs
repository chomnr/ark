use axum::async_trait;

use crate::app::{database::{postgres::PostgresDatabase, redis::RedisDatabase}, service::task::{message::{TaskRequest, TaskResponse}, TaskHandler}};

pub struct SessionTaskHandler;

#[async_trait]
impl TaskHandler<RedisDatabase> for SessionTaskHandler {
    async fn handle(redis: &RedisDatabase, task_request: TaskRequest) -> TaskResponse {
        todo!()
    }
}

// redis is used instead of postgres;
// user is cached
// the session is not cached.. (even though it is in the database.)