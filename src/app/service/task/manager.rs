use serde::Deserialize;

use crate::app::{
    database::postgres::PostgresDatabase,
    platform::iam::permission::task::PermissionTaskHandler,
    service::task::{
        message::{TaskStatus, TaskType},
        TaskHandler,
    },
};

use super::{
    message::{TaskRequest, TaskResponse},
    INBOUND, OUTBOUND,
};

/// A structure for handling tasks within the system.
///
/// This struct primarily interacts with a PostgreSQL database, represented by the `pg`
/// field. This struct is capable of sending tasks to be processed and listening for
/// tasks from various sources. It uses the PostgreSQL database for storing and retrieving
/// task-related information.
pub struct TaskManager {
    pg: PostgresDatabase,
}

impl TaskManager {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    /// Starts the listening process for task requests.
    ///
    /// This function initiates the task listening process by cloning the `PostgresDatabase` instance
    /// and passing it to `initialize_listener`. It's responsible for setting up the necessary resources
    /// and beginning the asynchronous task listening operation. The `pg_clone` is used to handle database
    /// operations within the listener.
    ///
    /// # Examples
    /// ```
    /// // Assuming `self` is an instance of the containing struct with a valid `pg` field
    /// self.listen();
    /// ```
    pub fn listen(self) {
        let pg_clone = self.pg.clone();
        self.initialize_listener(pg_clone);
    }

    /// Sends a task request and waits for its completion.
    ///
    /// This function first delegates the task of sending the request to `send_task_request` and
    /// then waits for the completion of the task using `wait_for_task_completion`. It ensures that
    /// the task request is properly dispatched and that a response is received before proceeding.
    /// The function returns a `TaskResponse` which encapsulates the result of the task.
    ///
    /// # Arguments
    /// - `task_request`: The `TaskRequest` object representing the task to be sent and processed.
    ///
    /// # Returns
    /// Returns a `TaskResponse` object which contains the outcome of the processed task.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_request` is a valid TaskRequest object
    /// let task_response = send(task_request);
    /// ```
    pub fn send<T: for<'a> Deserialize<'a>>(task_request: TaskRequest) -> TaskResponse {
        Self::send_task_request(&task_request);
        Self::wait_for_task_completion(&task_request)
    }

    /// Initializes and starts the task listener.
    ///
    /// This function spawns an asynchronous task to continuously listen for incoming task requests.
    /// It prints an initialization message and then enters a loop, waiting for new tasks. Upon receiving a task,
    /// it calls `process_incoming_request` to handle each task. The loop continues indefinitely, processing
    /// each incoming task request as they arrive.
    ///
    /// # Arguments
    /// - `pg_clone`: A cloned instance of `PostgresDatabase` used for handling database operations within tasks.
    ///
    /// # Examples
    /// ```
    /// // Assume `pg_clone` is a cloned instance of PostgresDatabase
    /// self.initialize_listener(pg_clone);
    /// ```
    fn initialize_listener(self, pg_clone: PostgresDatabase) {
        tokio::task::spawn(async move {
            let inbound_receiver = &INBOUND.1;
            println!("[ARK] Task initialized, now listening to incoming requests.");
            while let Ok(task_request) = inbound_receiver.recv() {
                self.process_incoming_request(&pg_clone, task_request).await;
            }
        });
    }

    /// Processes an incoming task request.
    ///
    /// This function logs the receipt of a new task and then delegates the handling of the task
    /// to the `handle_task_request` function.
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
        &self,
        pg_clone: &PostgresDatabase,
        task_request: TaskRequest,
    ) {
        println!(
            "[TASK] Successfully received a task from {}. Task type: {:?}.",
            task_request.task_id, task_request.task_type
        );
        self.handle_task_request(pg_clone, task_request).await;
    }

    /// Handles a given task request based on its type.
    ///
    /// This function matches the `task_type` of the provided `task_request` to determine
    /// the appropriate handler.
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
    async fn handle_task_request(&self, pg: &PostgresDatabase, task_request: TaskRequest) {
        match task_request.task_type {
            TaskType::Permission => {
                let task_response = PermissionTaskHandler::handle(pg, task_request).await;
                Self::send_task_response(task_response);
            }
        }
    }

    /// Sends a task response to the outbound channel.
    ///
    /// This function is responsible for dispatching the provided `task_response` to the outbound
    /// channel, making it available for further processing or logging.
    ///
    /// # Arguments
    /// - `task_response`: The `TaskResponse` object that encapsulates the result or outcome of a task.
    ///
    /// # Panics
    /// Panics if the send operation on the outbound channel fails.
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
    /// This function takes a reference to a `TaskRequest` and sends a cloned copy to the inbound channel.
    ///
    /// # Arguments
    /// - task_request: A reference to the TaskRequest object that needs to be sent.
    ///
    /// # Panics
    /// Panics if the send operation to the inbound channel fails.
    ///
    /// # Examples
    /// /// // Assuming `task_request` is a valid TaskRequest reference /// self.send_task_request(&task_request); ///
    fn send_task_request(task_request: &TaskRequest) {
        INBOUND.0.send(task_request.clone()).unwrap();
    }

    /// Waits for the completion of a specific task.
    ///
    /// This function iterates over the tasks in the OUTBOUND queue and matches them with the provided `task_request`.
    /// When it finds a matching task, it logs the outcome of the task using `log_task_outcome` and then returns the task.
    /// The function uses an `unreachable!()` statement, asserting that it should always find a matching task in the queue.
    ///
    /// # Arguments
    /// - `task_request`: A reference to the `TaskRequest` for which the completion is awaited.
    ///
    /// # Returns
    /// Returns the completed `TaskResponse`.
    ///
    /// # Panics
    /// Panics if no matching task is found in the OUTBOUND queue, which is considered an unexpected state.
    ///
    /// # Examples
    /// ```
    /// // Assuming `task_request` is a reference to a valid TaskRequest
    /// let task_response = wait_for_task_completion(&task_request);
    /// ```
    fn wait_for_task_completion(task_request: &TaskRequest) -> TaskResponse {
        for task in OUTBOUND.1.iter() {
            if task.task_id.eq(&task_request.task_id) {
                Self::log_task_outcome(&task);
                return task;
            }
        }
        unreachable!()
    }

    /// Logs the outcome of a task based on its response status.
    ///
    /// This function takes a reference to a `TaskResponse` and logs a message indicating the
    /// outcome of the task. It uses a match statement to differentiate between a completed task
    /// and a failed one. For a completed task, it logs a success message, and for a failed task,
    /// it logs an error message along with the first error detail from `task_error`.
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
