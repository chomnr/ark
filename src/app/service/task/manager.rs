use serde::{Deserialize, Serialize};
use tokio::task::{self, JoinHandle};

use crate::app::{
    database::{postgres::PostgresDatabase, redis::RedisDatabase},
    platform::iam::{permission::task::PermissionTaskHandler, role::task::RoleTaskHandler, user::task::UserTaskHandler},
    service::task::{
        message::{TaskStatus, TaskType},
        TaskHandler,
    },
};

use super::{
    message::{TaskRequest, TaskResponse},
    INBOUND, OUTBOUND, error::{TaskError, TaskResult},
};

/// A structure for handling tasks within the system.
pub struct TaskManager {
    pg: PostgresDatabase,
    redis: RedisDatabase
}

impl TaskManager {
    pub fn new(pg: PostgresDatabase, redis: RedisDatabase) -> Self {
        Self { pg, redis }
    }

    /// Starts the listening process for task requests.
    ///
    /// # Examples
    /// ```
    /// // Assuming `self` is an instance of the containing struct with a valid `pg` field
    /// self.listen();
    /// ```
    pub fn listen(self) {
        let pg_clone = self.pg.clone();
        Self::initialize_listener(pg_clone);
    }

    /// Sends a task request and waits for its completion.
    ///
    /// # Arguments
    /// - `task_request`: The `TaskRequest` object representing the task to be sent and processed.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_request` is a valid TaskRequest object
    /// let task_response = send(task_request);
    /// ```
    fn send(task_request: TaskRequest) -> TaskResponse {
        Self::send_task_request(&task_request);
        Self::wait_for_task_completion(&task_request)
    }

    /// Sends a task request and waits for its completion.
    ///
    /// # Arguments
    /// - `task_request`: The `TaskRequest` object representing the task to be sent and processed.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_request` is a valid TaskRequest object
    /// let task_response = send_async(task_request).await;
    /// ```
    fn send_async(task_request: TaskRequest) -> JoinHandle<TaskResponse> {
        let task_handle = task::spawn(async move {
            Self::send_task_request(&task_request);
            Self::wait_for_async_task_completion(&task_request).await.unwrap()
        });
        task_handle
    }

    /// Process task.
    ///
    /// # Arguments
    /// - `request`: A reference to the `TaskRequest` to process.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// Self::process_permission_task(request)
    /// ```
    pub fn process_task(request: TaskRequest) -> TaskResult<TaskStatus> {
        let task_response = Self::send(request);
        match task_response.task_status {
            TaskStatus::Completed => Ok(TaskStatus::Completed),
            TaskStatus::Failed => Err(TaskError::FailedToCompleteTask),
        }
    }

    pub fn process_task_with_result<T: for<'a> Deserialize<'a> + Serialize>(request: TaskRequest) -> TaskResult<T> {
        let task_response = Self::send(request);
        match task_response.task_status {
            TaskStatus::Completed => {
                let response = TaskResponse::intepret_response_result::<T>(&task_response);
                response
            },
            TaskStatus::Failed => Err(TaskError::FailedToCompleteTask),
        }
    }

    /// Initializes and starts the task listener.
    ///
    /// # Arguments
    /// - `pg_clone`: A cloned instance of `PostgresDatabase` used for handling database operations within tasks.
    ///
    /// # Examples
    /// ```
    /// // Assume `pg_clone` is a cloned instance of PostgresDatabase
    /// self.initialize_listener(pg_clone);
    /// ```
    fn initialize_listener(pg_clone: PostgresDatabase) {
        tokio::spawn(async move {
            let inbound_receiver = &INBOUND.1;
            println!("[ARK] Task initialized, now listening to incoming requests.");
            while let Ok(task_request) = inbound_receiver.recv() {
                Self::process_incoming_request(&pg_clone, task_request).await;
            }
        });
    }

    /// Processes an incoming task request.
    ///
    /// # Arguments
    /// - `pg_clone`: A reference to a cloned `PostgresDatabase` used for database operations.
    /// - `task_request`: The `TaskRequest` object representing the received task.
    ///
    /// # Examples
    /// ```
    /// // Assume `pg_clone` is a reference to a PostgresDatabase and `task_request` is a valid TaskRequest
    /// self.process_incoming_request(&pg_clone, task_request).await;
    /// ```
    async fn process_incoming_request(
        pg_clone: &PostgresDatabase,
        task_request: TaskRequest,
    ) {
        println!(
            "[TASK] Successfully received a task from {}. Task type: {:?}.",
            task_request.task_id, task_request.task_type
        );
        Self::handle_task_request(pg_clone, task_request).await;
    }

    /// Handles a given task request based on its type.
    ///
    /// # Arguments
    /// - `pg`: A reference to the `PostgresDatabase` used for database operations.
    /// - `task_request`: The `TaskRequest` object containing details about the task to be handled.
    ///
    /// # Examples
    /// ```
    /// // Assume `pg` is a reference to a PostgresDatabase and `task_request` is a valid TaskRequest
    /// self.handle_task_request(&pg, task_request).await;
    /// ```
    async fn handle_task_request(pg: &PostgresDatabase, task_request: TaskRequest) {
        match task_request.task_type {
            TaskType::Permission => {
                let task_response = PermissionTaskHandler::handle(pg, task_request).await;
                Self::send_task_response(task_response);
            }
            TaskType::Role => {
                let task_response = RoleTaskHandler::handle(pg, task_request).await;
                Self::send_task_response(task_response);
            },
            TaskType::User => {
                let task_response = UserTaskHandler::handle(pg, task_request).await;
                Self::send_task_response(task_response)
            },
        }
    }

    /// Sends a task response to the outbound channel.
    ///
    /// # Arguments
    /// - `task_response`: The `TaskResponse` object that encapsulates the result or outcome of a task.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_response` is a valid TaskResponse object
    /// send_task_response(task_response);
    /// ```
    fn send_task_response(task_response: TaskResponse) {
        OUTBOUND.0.send(task_response).unwrap();
    }

    /// Sends a task request to the inbound channel.
    ///
    /// # Arguments
    /// - task_request: A reference to the TaskRequest object that needs to be sent.
    ///
    /// # Examples
    /// /// // Assuming `task_request` is a valid TaskRequest reference /// self.send_task_request(&task_request); ///
    fn send_task_request(task_request: &TaskRequest) {
        INBOUND.0.send(task_request.clone()).unwrap();
    }

    /// Waits for the completion of a specific task.
    ///
    /// # Arguments
    /// - `task_request`: A reference to the `TaskRequest` for which the completion is awaited.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_request` is a reference to a valid TaskRequest
    /// let task_response = wait_for_task_completion(&task_request);
    /// ```
    fn wait_for_task_completion(task_request: &TaskRequest) -> TaskResponse {
        for task in OUTBOUND.1.iter(){
            if task.task_id.eq(&task_request.task_id) {
                Self::log_task_outcome(&task);
                return task;
            }
        }
        unreachable!()
    }

    /// Waits for the completion of a specific async task.
    ///
    /// # Arguments
    /// - `task_request`: A reference to the `TaskRequest` for which the completion is awaited.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_request` is a reference to a valid TaskRequest
    /// let task_response = wait_for_task_completion(&task_request);
    /// ```
    fn wait_for_async_task_completion(task_request: &TaskRequest) -> JoinHandle<TaskResponse> {
        let task_request_clone = task_request.clone(); // Clone the TaskRequest
        task::spawn(async move {
            for task in OUTBOUND.1.iter() {
                if task.task_id.eq(&task_request_clone.task_id) {
                    Self::log_task_outcome(&task);
                    return task;
                }
            }
            unreachable!()
        })
    }

    /// Logs the outcome of a task based on its response status.
    ///
    /// # Arguments
    /// - `task_response`: A reference to the `TaskResponse` object whose outcome is to be logged.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_response` is a reference to a valid TaskResponse
    /// log_task_outcome(&task_response);
    /// ```
    fn log_task_outcome(task_response: &TaskResponse) {
        match task_response.task_status {
            TaskStatus::Completed => println!(
                "[TASK] Task: {} successfully completed.",
                task_response.task_id
            ),
            TaskStatus::Failed => println!(
                "[TASK] Task: {} did not complete successfully. Error: {}",
                task_response.task_id, task_response.task_error[0]
            ),
        }
    }
}
