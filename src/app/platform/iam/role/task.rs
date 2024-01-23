use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::
        task::{
            error::TaskError,
            message::{TaskRequest, TaskResponse, TaskStatus},
            Task, TaskHandler,
        }
    ,
};

use super::model::Role;

pub struct RoleTaskHandler;

#[async_trait]
impl TaskHandler for RoleTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("role_create") {
            let payload =
                match TaskRequest::intepret_request_payload::<RoleCreateTask>(&task_request) {
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
            let payload =
                match TaskRequest::intepret_request_payload::<RoleUpdateTask>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return RoleUpdateTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("role_delete") {
            let payload =
                match TaskRequest::intepret_request_payload::<RoleDeleteTask>(&task_request) {
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

        if task_request.task_action.eq("role_add_permission") {
            todo!()
        }

        if task_request.task_action.eq("role_preload_cache") {
            let payload =
                match TaskRequest::intepret_request_payload::<RolePreloadCache>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return RolePreloadCache::run(pg, task_request, payload).await;
        }

        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

// create role

#[derive(Serialize, Deserialize)]
pub struct RoleCreateTask {
    pub role_id: String,
    pub role_name: String,
    pub role_permissions: Vec<String>,
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
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: RoleCreateTask,
    ) -> TaskResponse {
        // somehow integrate role_permissions
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO iam_roles (id, role_name) VALUES ($1, $2)")
            .await
            .unwrap();
        match pool
            .execute(&stmt, &[&param.role_id, &param.role_name])
            .await
        {
            Ok(_) => {
                let role = Role::from(param);
                //RoleCache::add(role.clone());
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

// role update permission
#[derive(Serialize, Deserialize)]
pub(super) struct RoleUpdateTask {
    pub search_by: String,
    pub update_for: String,
    pub value: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, RoleUpdateTask> for RoleUpdateTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: RoleUpdateTask,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        if param.update_for.eq_ignore_ascii_case("id") {
            return TaskResponse::throw_failed_response(
                request,
                vec![TaskError::FieldNotMutable.to_string()],
            );
        }

        let stmt = match pool
            .prepare(
                format!(
                    "UPDATE iam_roles
                SET {} = $1
                WHERE id = $2
                   OR role_name = $2",
                    param.update_for
                )
                .as_str(),
            )
            .await
        {
            Ok(v) => v,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleFieldNotFound.to_string()],
                )
            }
        };
        match pool.execute(&stmt, &[&param.value, &param.search_by]).await {
            Ok(v) => {
                if v != 0 {
                    //RoleCache::update(&param.search_by, &param.update_for, &param.value);
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
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: RoleDeleteTask,
    ) -> TaskResponse {
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
                    //RoleCache::remove(&param.identifier);
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

#[derive(Serialize, Deserialize)]
pub(super) struct RolePreloadCache;
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, RolePreloadCache> for RolePreloadCache {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: RolePreloadCache,
    ) -> TaskResponse {
        todo!();

        
        let pool = db.pool.get().await.unwrap();
        let stmt = pool.prepare("SELECT * FROM iam_roles").await.unwrap();

        match pool.query(&stmt, &[]).await {
            Ok(rows) => {
                //let mut amt_items = 0;
                //let mut role_permissions = Vec::new();
                for row in rows {
                    let stmt = pool
                        .prepare("SELECT permission_id FROM iam_role_permission WHERE role_id = $1")
                        .await
                        .unwrap();
                    match pool.query(&stmt, &[&row.get::<usize, String>(0)]).await {
                        Ok(permissions) => {
                            for permission in permissions {
                                //role_permissions.push(permission.get(0))
                            }
                        }
                        Err(er) => {
                            // nothing is needed to do.
                            println!("{}", er);
                        }
                    }
                    // add to cache.
                    //RoleCache::add(Role::new(row.get(0), row.get(1), role_permissions.clone()));
                    //amt_items += 1;
                }
                //println!("[CACHE] cached {} for role cache.", amt_items);
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    String::default(),
                    Vec::default(),
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionFailedToPreload.to_string()],
                )
            }
        }
    }
}
