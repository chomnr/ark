use axum::async_trait;

use crate::app::{
    database::postgres::PostgresDatabase,
    service::task::{
        message::{TaskRequest, TaskResponse},
        TaskHandler,
    },
};

pub struct RoleTaskHandler;

#[async_trait]
impl TaskHandler for RoleTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("role_create") {
            todo!()
        }

        if task_request.task_action.eq("role_update") {
            todo!()
        }

        if task_request.task_action.eq("role_delete") {
            todo!()
        }

        if task_request.task_action.eq("role_read") {
            todo!()
        }

        if task_request.task_action.eq("role_preload_cache") {
            todo!()
        }
        todo!()
    }
}
