use axum::async_trait;
use serde::{Serialize, Deserialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::{task::{
        message::{TaskRequest, TaskResponse, TaskStatus},
        Task, TaskHandler, error::TaskError,
    }, cache::LocalizedCache}, platform::iam::permission::model::Permission,
};

use super::{model::Role, RoleCache};

pub struct RoleTaskHandler;

#[async_trait]
impl TaskHandler for RoleTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("role_create") {
            let payload = match TaskRequest::intepret_request_payload::<RoleCreateTask>(
                &task_request,
            ) {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return RoleCreateTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("role_update") {
            todo!()
        }

        if task_request.task_action.eq("role_delete") {
            let payload = match TaskRequest::intepret_request_payload::<RoleDeleteTask>(
                &task_request,
            ) {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return RoleDeleteTask::run(pg, task_request, payload).await;
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

// create role

#[derive(Serialize, Deserialize)]
pub struct RoleCreateTask {
    pub role_id: String,
    pub role_name: String,
    pub role_permissions: Vec<Permission>
}

impl From<Role> for RoleCreateTask {
    fn from(role: Role) -> Self {
        Self {
            role_id: role.role_id,
            role_name: role.role_name,
            role_permissions: role.role_permissions,
        }
    }
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, RoleCreateTask> for RoleCreateTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: RoleCreateTask) -> TaskResponse {
        // somehow integrate role_permissions
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare(
                "INSERT INTO iam_roles (id, role_name) VALUES ($1, $2)",
            )
            .await
            .unwrap();
        match pool
            .execute(
                &stmt,
                &[
                    &param.role_id,
                    &param.role_name,
                ],
            )
            .await
        {
            Ok(_) => {
                let role = Role::from(param);
                RoleCache::add(role.clone());
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    role,
                    Vec::default(),
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleDuplication.to_string()],
                )
            }
        }
    }
}

// update role
#[derive(Serialize, Deserialize)]
pub(super) struct RoleDeleteTask {
    pub identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, RoleDeleteTask> for RoleDeleteTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: RoleDeleteTask) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
            let stmt = pool
                .prepare(
                    "DELETE FROM iam_roles
                    WHERE id = $1
                       OR role_name = $1",
                )
                .await
                .unwrap();
            match pool.execute(&stmt, &[&param.identifier]).await {
                Ok(v) => {
                    if v != 0 {
                        RoleCache::remove(&param.identifier);
                        return TaskResponse::compose_response(
                            request,
                            TaskStatus::Completed,
                            param,
                            Vec::default(),
                        );
                    }
                    return TaskResponse::throw_failed_response(
                        request,
                        vec![TaskError::RoleNotFound.to_string()],
                    );
                }
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        request,
                        vec![TaskError::TaskInternalError.to_string()],
                    )
                }
            }
    }
}