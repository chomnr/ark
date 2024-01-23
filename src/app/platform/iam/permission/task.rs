use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::{
        cache::{manager::CacheManager, LocalizedCache},
        task::{
            error::TaskError,
            message::{TaskRequest, TaskResponse, TaskStatus},
            Task, TaskHandler,
        },
    },
};

use super::{model::Permission, PermissionCache};

pub struct PermissionTaskHandler;

#[async_trait]
impl TaskHandler for PermissionTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        if task_request.task_action.eq("permission_create") {
            let payload = match TaskRequest::intepret_request_payload::<PermissionCreateTask>(
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
            return PermissionCreateTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("permission_delete") {
            let payload = match TaskRequest::intepret_request_payload::<PermissionDeleteTask>(
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
            return PermissionDeleteTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("permission_update") {
            let payload = match TaskRequest::intepret_request_payload::<PermissionUpdateTask>(
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
            return PermissionUpdateTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("permission_read") {
            let payload =
                match TaskRequest::intepret_request_payload::<PermissionReadTask>(&task_request) {
                    Ok(p) => p,
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            task_request,
                            vec![TaskError::FailedToInterpretPayload.to_string()],
                        )
                    }
                };
            return PermissionReadTask::run(pg, task_request, payload).await;
        }

        if task_request.task_action.eq("permission_preload_cache") {
            let payload = match TaskRequest::intepret_request_payload::<PermissionPreloadCache>(
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
            return PermissionPreloadCache::run(pg, task_request, payload).await;
        }

        return TaskResponse::throw_failed_response(
            task_request,
            vec![TaskError::FailedToFindAction.to_string()],
        );
    }
}

/// Represents a task for creating a permission.
///
/// This struct does not hold any data itself and serves as a marker for implementing the `Task` trait,
/// specifically for creating a permission in a PostgreSQL database. The task takes a `Permission` object
/// as a parameter and returns a `TaskResult<bool>` indicating the success or failure of the operation.
///
/// # Examples
///
/// ```
/// #[async_trait]
/// impl Task<Permission, PostgresDatabase, bool> for CreatePermissionTask {
///     async fn run(pg: PostgresDatabase, param: Permission) -> TaskResult<bool> {
///         // Implementation goes here
///     }
/// }
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub(super) struct PermissionCreateTask {
    pub permission_id: String,
    pub permission_name: String,
    pub permission_key: String,
}

impl From<Permission> for PermissionCreateTask {
    fn from(perm: Permission) -> Self {
        Self {
            permission_id: perm.permission_id,
            permission_name: perm.permission_name,
            permission_key: perm.permission_key,
        }
    }
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, PermissionCreateTask> for PermissionCreateTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: PermissionCreateTask,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare(
                "INSERT INTO iam_permissions (id, permission_name, permission_key) VALUES ($1, $2, $3)",
            )
            .await
            .unwrap();
        match pool
            .execute(
                &stmt,
                &[
                    &param.permission_id,
                    &param.permission_name,
                    &param.permission_key,
                ],
            )
            .await
        {
            Ok(_) => {
                PermissionCache::add(Permission::from(param.clone()));
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
                    vec![TaskError::PermissionDuplication.to_string()],
                )
            }
        }
    }
}

/// Represents a task for deleting a permission.
///
/// This struct does not hold any data itself and serves as a marker for implementing the `Task` trait,
/// specifically for deleting a permission in a PostgreSQL database. The task typically takes a `Permission` object
/// identifier as a parameter and returns a `TaskResult<bool>` indicating the success or failure of the deletion operation.
///
/// # Examples
///
/// ```
/// #[async_trait]
/// impl Task<PermissionIdentifier, PostgresDatabase, bool> for PermissionDeleteTask {
///     async fn run(pg: PostgresDatabase, param: PermissionIdentifier) -> TaskResult<bool> {
///         // Implementation goes here
///     }
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub(super) struct PermissionDeleteTask {
    pub identifier: String,
}
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, PermissionDeleteTask> for PermissionDeleteTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: PermissionDeleteTask,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare(
                "DELETE FROM iam_permissions
                WHERE id = $1
                   OR permission_name = $1
                   OR permission_key = $1",
            )
            .await
            .unwrap();
        match pool.execute(&stmt, &[&param.identifier]).await {
            Ok(v) => {
                if v != 0 {
                    PermissionCache::remove(&param.identifier);
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        param,
                        Vec::default(),
                    );
                }
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionNotFound.to_string()],
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

/// Represents a task for updating a permission.
///
/// This struct holds the criteria for finding the permission to update (`search_for`) and the new permission data (`new_permission`).
/// It serves as a marker for implementing the `Task` trait, specifically for updating a permission in a PostgreSQL database.
/// The task takes a `PermissionUpdateTask` object as a parameter, which contains the search criteria and the new permission data,
/// and returns a `TaskResponse` indicating the success or failure of the update operation.
///
/// # Examples
///
/// ```
/// #[async_trait]
/// impl Task<PostgresDatabase, TaskRequest, PermissionUpdateTask> for PermissionUpdateTask {
///     async fn run(
///         db: &PostgresDatabase,
///         request: TaskRequest,
///         param: PermissionUpdateTask,
///     ) -> TaskResponse {
///         // Implementation for updating a permission
///     }
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub(super) struct PermissionUpdateTask {
    pub search_by: String,
    pub update_for: String,
    pub value: String,
}
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, PermissionUpdateTask> for PermissionUpdateTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: PermissionUpdateTask,
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
                    "UPDATE iam_permissions
                SET {} = $1
                WHERE id = $2
                   OR permission_name = $2
                   OR permission_key = $2",
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
                    vec![TaskError::PermissionFieldNotFound.to_string()],
                )
            }
        };

        match pool.execute(&stmt, &[&param.value, &param.search_by]).await {
            Ok(v) => {
                if v != 0 {
                    PermissionCache::update(&param.search_by, &param.update_for, &param.value);
                    return TaskResponse::compose_response(
                        request,
                        TaskStatus::Completed,
                        param,
                        Vec::default(),
                    );
                }
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionNotFound.to_string()],
                );
            }
            Err(_) => {
                return TaskResponse::throw_failed_response(
                    request,
                    vec![TaskError::PermissionDuplication.to_string()],
                )
            }
        }
    }
}

/// Represents a task for reading a permission.
#[derive(Serialize, Deserialize)]
pub(super) struct PermissionReadTask {
    pub identifier: String,
}

#[async_trait]
impl Task<PostgresDatabase, TaskRequest, PermissionReadTask> for PermissionReadTask {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: PermissionReadTask,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();

        match PermissionCache::get(&param.identifier) {
            Ok(permission) => {
                CacheManager::notify_cache_hit("PermissionCache", &param.identifier, &request.task_id);
                return TaskResponse::compose_response(
                    request,
                    TaskStatus::Completed,
                    permission,
                    Vec::default(),
                );
            }
            Err(_) => {
                let stmt = pool
                    .prepare(
                        "SELECT * FROM iam_permissions WHERE id = $1
        OR permission_name = $1
        OR permission_key = $1",
                    )
                    .await
                    .unwrap();
                match pool.query_one(&stmt, &[&param.identifier]).await {
                    Ok(row) => {
                        CacheManager::notify_cache_miss("PermissionCache", &param.identifier, &request.task_id);
                        let permission = Permission::new(row.get(0), row.get(1), row.get(2));
                        PermissionCache::add(permission.clone());
                        return TaskResponse::compose_response(
                            request,
                            TaskStatus::Completed,
                            permission,
                            Vec::default(),
                        );
                    }
                    Err(_) => {
                        return TaskResponse::throw_failed_response(
                            request,
                            vec![TaskError::PermissionNotFound.to_string()],
                        )
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct PermissionPreloadCache;
#[async_trait]
impl Task<PostgresDatabase, TaskRequest, PermissionPreloadCache> for PermissionPreloadCache {
    async fn run(
        db: &PostgresDatabase,
        request: TaskRequest,
        param: PermissionPreloadCache,
    ) -> TaskResponse {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool.prepare("SELECT * FROM iam_permissions").await.unwrap();

        match pool.query(&stmt, &[]).await {
            Ok(rows) => {
                let mut amt_items = 0;
                for row in rows {
                    PermissionCache::add(Permission::new(row.get(0), row.get(1), row.get(2)));
                    amt_items += 1;
                }
                println!("[CACHE] cached {} for permission cache.", amt_items);
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
