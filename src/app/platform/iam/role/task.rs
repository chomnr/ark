use crate::app::{
    platform::iam::permission::{cache::PermissionCache, manager::PermissionManager},
    service::cache::{notify_cache_hit, notify_cache_miss, LocalizedCache},
};
use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::task::{
        error::TaskError,
        message::{TaskRequest, TaskResponse, TaskStatus},
        Task, TaskHandler,
    },
};

use super::{cache::RoleCache, model::Role};

pub struct RoleTaskHandler;

#[async_trait]
impl TaskHandler<PostgresDatabase> for RoleTaskHandler {
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
            let payload = match TaskRequest::intepret_request_payload::<RoleReadTask>(&task_request)
            {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return RoleReadTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("role_add_permission") {
            let payload = match TaskRequest::intepret_request_payload::<RolePermissionLinkToRole>(
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
            return RolePermissionLinkToRole::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("role_delete_permission") {
            let payload = match TaskRequest::intepret_request_payload::<
                RolePermissionDeleteLinkToRole,
            >(&task_request)
            {
                Ok(p) => p,
                Err(_) => {
                    return TaskResponse::throw_failed_response(
                        task_request,
                        vec![TaskError::FailedToInterpretPayload.to_string()],
                    )
                }
            };
            return RolePermissionDeleteLinkToRole::run(pg, task_request, payload).await;
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
                   OR role_name = $2
                   RETURNING *;",
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
        match pool
            .query_one(&stmt, &[&param.value, &param.search_by])
            .await
        {
            Ok(v) => {
                if v.len() != 0 {
                    let old_role = RoleCache::get(v.get(0)).unwrap();
                    RoleCache::remove(&old_role.role_name).unwrap();
                    // TODO THIS....
                    // TODO THIS....
                    // TODO THIS....
                    // TODO THIS....
                    // NEED TO BE ABLE TO ADD SPECIFIC PERMISSIONS TO THIS
                    RoleCache::add(Role::new(
                        v.get(0),
                        v.get(1),
                        RoleCache::get(v.get(0)).unwrap().role_permissions,
                    ));
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
                    RoleCache::remove(&param.identifier).unwrap();
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
pub(super) struct RoleReadTask {
    pub identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, RoleReadTask> for RoleReadTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest, param: RoleReadTask) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        match RoleCache::get(&param.identifier) {
            Ok(role) => {
                notify_cache_hit("RoleCache", "RoleReadTask", &request.task_id);
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    role,
                    Vec::default(),
                );
            }
            Err(_) => {
                let stmt = pool
                    .prepare(
                        "SELECT * FROM iam_roles WHERE id = $1
        OR role_name = $1;",
                    )
                    .await
                    .unwrap();
                match pool.query_one(&stmt, &[&param.identifier]).await {
                    Ok(row) => {
                        notify_cache_miss("RoleCache", "RoleReadTask", &request.task_id);
                        let mut role_permissions: Vec<String> = Vec::new();
                        let stmt = pool
                            .prepare(
                                "SELECT permission_id FROM iam_role_permission WHERE role_id = $1",
                            )
                            .await
                            .unwrap();
                        match pool.query(&stmt, &[&row.get::<usize, String>(0)]).await {
                            Ok(permissions) => {
                                for permission in permissions {
                                    role_permissions.push(permission.get(0))
                                }
                            }
                            Err(er) => {
                                println!("{}", er);
                            }
                        }
                        let role = Role::new(row.get(0), row.get(1), role_permissions);
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
                            vec![TaskError::RoleNotFound.to_string()],
                        )
                    }
                }
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
        let pool = db.pool.get().await.unwrap();
        let stmt = pool.prepare("SELECT * FROM iam_roles").await.unwrap();

        match pool.query(&stmt, &[]).await {
            Ok(rows) => {
                let mut amt_items = 0;
                for row in rows {
                    let mut role_permissions: Vec<String> = Vec::new();
                    let stmt = pool
                        .prepare("SELECT permission_id FROM iam_role_permission WHERE role_id = $1")
                        .await
                        .unwrap();
                    match pool.query(&stmt, &[&row.get::<usize, String>(0)]).await {
                        Ok(permissions) => {
                            for permission in permissions {
                                role_permissions.push(permission.get(0))
                            }
                        }
                        Err(er) => {
                            println!("{}", er);
                        }
                    }
                    RoleCache::add(Role::new(row.get(0), row.get(1), role_permissions.clone()));
                    amt_items += 1;
                }
                println!("[CACHE] cached {} role(s) cache.", amt_items);
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

#[derive(Serialize, Deserialize)]
pub(super) struct RolePermissionLinkToRole {
    pub role_id: String,
    pub permission_id: String,
}
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, RolePermissionLinkToRole> for RolePermissionLinkToRole {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: RolePermissionLinkToRole,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();

        let role_to_id = match RoleCache::get(&param.role_id) {
            Ok(v) => v.role_id,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleNotFound.to_string()],
                )
            }
        };
        let permission_to_id = match PermissionCache::get(&param.permission_id) {
            Ok(v) => v.permission_id,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionNotFound.to_string()],
                )
            }
        };

        let stmt = pool
            .prepare("INSERT INTO iam_role_permission (role_id, permission_id) VALUES ($1, $2)")
            .await
            .unwrap();
        match pool.query(&stmt, &[&role_to_id, &permission_to_id]).await {
            Ok(_) => {
                // overrides the existing role (because value is the value is a shared state(arc))
                // it should reflect throughout the rest of the application
                let mut role = RoleCache::get(&param.role_id).unwrap();
                role.role_permissions.push(permission_to_id);
                RoleCache::add(role);
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    param,
                    Vec::default(),
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleLinkFailedToLink.to_string()],
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct RolePermissionDeleteLinkToRole {
    pub role_id: String,
    pub permission_id: String,
}
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, RolePermissionDeleteLinkToRole>
    for RolePermissionDeleteLinkToRole
{
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: RolePermissionDeleteLinkToRole,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        // role to id conversion incase the param is not an id.
        let role_to_id = match RoleCache::get(&param.role_id) {
            Ok(v) => v.role_id,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleNotFound.to_string()],
                )
            }
        };
        let permission_to_id = match PermissionCache::get(&param.permission_id) {
            Ok(v) => v.permission_id,
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionNotFound.to_string()],
                )
            }
        };
        // conversion ends here..
        let stmt = pool
            .prepare(
                "DELETE FROM iam_role_permission
            WHERE role_id = $1
               AND permission_id = $2",
            )
            .await
            .unwrap();
        match pool.query(&stmt, &[&role_to_id, &permission_to_id]).await {
            Ok(_) => {
                // overrides the existing role (because value is the value is a shared state(arc))
                // it should reflect throughout the rest of the application
                let mut role = RoleCache::get(&param.role_id).unwrap();
                role.role_permissions
                    .retain(|permission| permission != &permission_to_id);
                RoleCache::add(role);
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    param,
                    Vec::default(),
                );
            }
            Err(er) => {
                println!("{}", er);
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::RoleLinkFailedToLink.to_string()],
                );
            }
        }
    }
}
