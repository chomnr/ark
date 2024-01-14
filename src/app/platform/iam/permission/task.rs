use axum::async_trait;

use crate::app::{
    database::{postgres::PostgresDatabase, redis::RedisDatabase},
    services::task::{
        error::{TaskError, TaskResult},
        model::TaskHandler,
    },
};

pub struct PermissionTaskHandler;

#[async_trait]
impl TaskHandler for PermissionTaskHandler {
    async fn handle<T>(
        pg: Option<PostgresDatabase>,
        redis: Option<RedisDatabase>,
        task_action: &str,
    ) -> TaskResult<T> {
        let binding = pg.unwrap();
        let pool = binding.pool.get().await.unwrap();
        // handle all possible outcomes
        if task_action.eq("permission_create") {
            // create permission
            //pool.query(statement, params)
        }
        if task_action.eq("permission_delete") {
            // update permission
        }
        if task_action.eq("permission_update") {
            // delete permission
        }
        Err(TaskError::TaskInvalid)
    }
}
