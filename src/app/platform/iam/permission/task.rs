use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    platform::iam::permission::model::Permission,
    service::task::{
        error::TaskError,
        message::{TaskRequest, TaskResponse, TaskStatus},
        Task, TaskHandler,
    },
};

#[derive(Serialize, Deserialize)]
pub struct PermissionTask<T: Serialize> {
    pub action: String,
    pub param: T,
}

pub struct PermissionTaskHandler;

#[async_trait]
impl TaskHandler for PermissionTaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse {
        let payload = TaskRequest::intepret_request_payload::<PermissionTask<Permission>>(&task_request).unwrap();
        if payload.action.eq("permission_create") {
            return PermissionCreateTask::run(pg, task_request).await;
        }
        return TaskResponse::throw_failed_response(task_request, vec![TaskError::FailedToFindAction.to_string()]);
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
///
/// In this implementation, `run` is an asynchronous function that should contain the logic for creating
/// a new permission in the database. The result of this operation is encapsulated in `TaskResult<bool>`.
struct PermissionCreateTask;
#[async_trait]
impl Task<TaskRequest, PostgresDatabase> for PermissionCreateTask {
    async fn run(db: &PostgresDatabase, request: TaskRequest) -> TaskResponse {
        let payload =
            match TaskRequest::intepret_request_payload::<PermissionTask<Permission>>(&request) {
                Ok(v) => v,
                Err(er) => {
                    return TaskResponse::throw_failed_response(request, vec![er.to_string()])
                }
            };
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
                    &payload.param.permission_id,
                    &payload.param.permission_name,
                    &payload.param.permission_key,
                ],
            )
            .await
        {
            Ok(_) => return TaskResponse::compose_response(request, TaskStatus::Completed, payload.param, Vec::default()),
            Err(er) => return TaskResponse::throw_failed_response(request, vec![er.to_string()]),
        }
    }
}

/*

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
///
/// In this implementation, `run` is an asynchronous function that should contain the logic for creating
/// a new permission in the database. The result of this operation is encapsulated in `TaskResult<bool>`.
struct CreatePermissionTask;
#[async_trait]
impl Task<Permission, PostgresDatabase, Permission> for CreatePermissionTask {
    async fn run(db: &PostgresDatabase, param: Permission) -> TaskResult<Permission> {
        let pool = db.pool.get().await.unwrap();
        let stmt = pool
            .prepare(
                "INSERT INTO iam_permissions (id, permission_name, permission_key) VALUES ($1, $2, $3)",
            )
            .await
            .unwrap();
        pool.execute(
            &stmt,
            &[
                &param.permission_id,
                &param.permission_name,
                &param.permission_key,
            ],
        )
        .await
        .map(|_| param)
        .map_err(|_| TaskError::TaskFailure)
    }
}


static INBOUND: Lazy<(Sender<TaskMessage>, Receiver<TaskMessage>)> = Lazy::new(|| unbounded());
static OUTBOUND: Lazy<(Sender<TaskMessageResult>, Receiver<TaskMessageResult>)> =
    Lazy::new(|| unbounded());

pub struct PermissionHandler;

impl PermissionHandler {
    pub fn listen(pg: PostgresDatabase) {
        tokio::task::spawn(async move {
            println!("[ARK] permission task initialized, now receiving tasks.");
            for task in INBOUND.1.iter() {
                PermissionHandler::handle(pg.clone(), task).await;
            }
        });
    }

    pub fn send<R: for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static>(
        task: TaskMessage,
    ) -> JoinHandle<TaskResult<R>> {
        INBOUND.0.send(task.clone()).unwrap();
        tokio::task::spawn(async move {
            for result in OUTBOUND.1.iter() {
                if result.task_id.eq(&task.task_id) {
                    return if !result.task_completed {
                        println!("failed");
                        Err(TaskError::TaskFailure)
                    } else {
                        println!("success");
                        let value: R = serde_json::from_str(&task.task_message).unwrap();
                        Ok(value)
                    };
                }
            }
            Err(TaskError::TaskFailure)
        })
    }

    fn send_result(task_result: TaskMessageResult) {
        OUTBOUND.0.send(task_result).unwrap();
    }
}

#[async_trait]
impl TaskHandler for PermissionHandler {
    async fn handle(pg: PostgresDatabase, task: TaskMessage) {
        if task.task_action.eq("permission_create") {
            // create permission
            let perm: Permission = serde_json::from_str(&task.task_message).unwrap();
            match CreatePermissionTask::run(&pg, perm).await {
                Ok(v) => {
                    let out = TaskMessageResult::compose(&task.task_id, true, v);
                    Self::send_result(out);
                }
                Err(_) => {
                    let out = TaskMessageResult::compose(&task.task_id, true, String::default());
                    Self::send_result(out);
                }
            }
        }
    }
}
*/
