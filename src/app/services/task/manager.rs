use serde::Deserialize;
use tokio::task;

use crate::app::{
    database::{postgres::PostgresDatabase, redis::RedisDatabase},
    platform::iam::permission::{model::Permission, task::PermissionTaskHandler},
    services::task::TASK_CHANNEL,
};

use super::{
    error::{TaskError, TaskResult},
    model::{TaskHandler, TaskMessage, TaskMessageResult, TaskResultStatus},
    TASK_RESULT_CHANNEL,
};

/// `TaskManager`related to task handling, such as task distribution, monitoring, and result management.
pub struct TaskManager {
    pg: PostgresDatabase,
    redis: RedisDatabase,
}

impl TaskManager {
    /// Constructs a new instance of the struct using specified PostgreSQL and Redis database connections.
    ///
    /// # Arguments
    ///
    /// * `pg` - A `PostgresDatabase` connection.
    /// * `redis` - A `RedisDatabase` connection.
    ///
    /// # Examples
    ///
    /// ```
    /// let pg_connection = PostgresDatabase::new(/* PostgresConfig */);
    /// let redis_connection = RedisDatabase::new(/* RedisConfig */);
    ///
    /// let my_struct = TaskManager::with_databases(pg_connection, redis_connection);
    /// ```
    pub fn with_databases(pg: PostgresDatabase, redis: RedisDatabase) -> Self {
        Self { pg, redis }
    }

    /// Asynchronously listens to a channel and processes tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let my_struct = MyStruct::new(); // Assume MyStruct is initialized here
    ///     my_struct.listen_for_tasks().await;
    /// }
    /// ```
    pub async fn listen_for_tasks(&self) {
        let pg_clone = self.pg.clone();
        task::spawn(async move {
            println!("[ARK] task_channel initalized, now listening to incoming tasks.");
            for task in TASK_CHANNEL.1.iter() {
                Self::process_task(&pg_clone, task).await;
            }
        });
    }

    /// Sends a `TaskMessage` to the task channel.
    ///
    /// # Arguments
    ///
    /// * `task_message` - The `TaskMessage` to be sent to the task channel.
    ///
    /// # Examples
    ///
    /// ```
    /// let message = TaskMessage::new(/* ... */);
    /// TaskManager::send(message);
    /// // The message is now sent to the task channel for further processing.
    /// ```
    pub fn send<T: for<'a> Deserialize<'a>>(task_message: TaskMessage) -> TaskResult<T> {
        TASK_CHANNEL.0.send(task_message.clone()).unwrap();
        for task in TASK_RESULT_CHANNEL.1.iter() {
            if task.task_id.eq(&task_message.task_id) {
                let result: T = serde_json::from_str(&task.result).unwrap();
                return Ok(result);
            }
        }
        return Err(TaskError::TaskWentWrong);
    }

    /// Sends a `TaskMessageResult` to the task channel.
    ///
    /// # Arguments
    ///
    /// * `task_message` - The `TaskMessageResult` to be sent to the task channel.
    ///
    /// # Examples
    ///
    /// ```
    /// let message = TaskMessage::new(/* ... */);
    /// TaskManager::send(message);
    /// // The message is now sent to the task channel for further processing.
    /// ```
    fn send_result(task_result: TaskMessageResult) {
        TASK_RESULT_CHANNEL.0.send(task_result).unwrap();
    }

    /// Processes a task.
    ///
    /// # Arguments
    ///
    /// * `pg` - A `PostgresDatabase` connection.
    /// * `task` - A TaskMessage.
    ///
    /// # Examples
    ///
    /// ```
    /// fn main() {
    ///     // Task processing
    ///     process_task();
    ///     // The task has been processed
    /// }
    /// ```
    async fn process_task(pg: &PostgresDatabase, task: TaskMessage) {
        let process_result = match task.task_type {
            super::model::TaskType::Permission => {
                Self::process_permission_specific_task(pg, &task).await
            }
        };

        match process_result {
            Ok(_) => println!(
                "[ARK] successfully processed task: '{}' action: {}",
                task.task_id, task.task_action
            ),
            Err(err) => println!(
                "[ARK] failed to process task: '{}' action: {} error: {}",
                task.task_id, task.task_action, err
            ),
        }
    }

    /// Processes a permission-specific task and returns a result.
    ///
    /// # Arguments
    ///
    /// * `task` - A `TaskMessage` object that contains the details of the task requiring specific permissions.
    ///
    /// # Examples
    ///
    /// ```
    /// fn main() {
    ///     let task = TaskMessage::new(/* parameters to create a TaskMessage */);
    ///     match process_permission_specific_task(task) {
    ///         Ok(()) => println!("Task successfully processed"),
    ///         Err(e) => println!("Error processing task: {:?}", e),
    ///     }
    /// }
    /// ```
    async fn process_permission_specific_task(
        pg: &PostgresDatabase,
        task: &TaskMessage,
    ) -> TaskResult<()> {
        let action = &task.task_action;
        match PermissionTaskHandler::handle::<Permission>(Some(pg.clone()), None, &action).await {
            Ok(v) => {
                Self::send_result(TaskMessageResult::compose(
                    &task.task_id,
                    TaskResultStatus::SUCCESSFUL,
                    Some(v),
                ));
                return Ok(());
            }
            Err(_) => {
                let permission: Permission = serde_json::from_str(&task.task_message).unwrap();
                Self::send_result(TaskMessageResult::compose(
                    &task.task_id,
                    TaskResultStatus::FAILED,
                    Some(permission),
                ));
                return Err(TaskError::TaskWentWrong);
            }
        }
    }
}