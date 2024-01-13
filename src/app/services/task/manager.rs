use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tokio::task;

use crate::app::{
    database::{postgres::PostgresDatabase, redis::RedisDatabase},
    platform::iam::user::task::UserCreateTask,
};

use super::{error::TaskResult, model::TaskMessage};

static TASK_CHANNEL: Lazy<(Sender<TaskMessage>, Receiver<TaskMessage>)> = Lazy::new(|| unbounded());
static TASK_RESULT_CHANNEL: Lazy<(Sender<TaskMessage>, Receiver<TaskMessage>)> =
    Lazy::new(|| unbounded());

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
    ///     my_struct.listen().await;
    /// }
    /// ```
    pub async fn listen(&self) {
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
    pub fn send(task_message: TaskMessage) {
        TASK_CHANNEL.0.send(task_message).unwrap();
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
        match task.task_type {
            super::model::TaskType::Permission => {
                match Self::process_permission_specific_task(pg, &task).await {
                    Ok(_) => todo!(), /* Send to receiver with the necessary parameters saying it was a success */
                    Err(_) => todo!(), /* Sends to receiver saying it failed... */
                }
            }
            super::model::TaskType::Role => {
                match Self::process_role_specific_task(pg, &task).await {
                    Ok(_) => todo!(), /* Send to receiver with the necessary parameters saying it was a success */
                    Err(_) => todo!(), /* Sends to receiver saying it failed... */
                }
            }
            super::model::TaskType::User => {
                match Self::process_user_specific_task(pg, &task).await {
                    Ok(_) =>  println!("[ARK] successfully processed task: '{}' action: {}", &task.task_id, &task.task_action), /* Send to receiver with the necessary parameters saying it was a success */
                    Err(_) => todo!(), /* Sends to receiver saying it failed... */
                }
            }
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
        todo!()
    }

    /// Processes a role-specific task and returns a result.
    ///
    /// # Arguments
    ///
    /// * `task` - A `TaskMessage` containing the details of the role-specific task.
    ///
    /// # Examples
    ///
    /// ```
    /// fn main() {
    ///     let task = TaskMessage::new(/* parameters to create a TaskMessage */);
    ///     match process_role_specific_task(task) {
    ///         Ok(()) => println!("Task successfully processed"),
    ///         Err(e) => println!("Error processing task: {:?}", e),
    ///     }
    /// }
    /// ```
    async fn process_role_specific_task(
        pg: &PostgresDatabase,
        task: &TaskMessage,
    ) -> TaskResult<()> {
        todo!()
    }

    /// Processes a user-specific task based on the provided task message.
    ///
    /// # Arguments
    ///
    /// * `task` - A `TaskMessage` object representing the task to be processed.
    ///
    /// # Examples
    ///
    /// ```
    /// fn main() {
    ///     let task = TaskMessage::compose(/* parameters to create a TaskMessage */);
    ///     process_user_specific_task(task);
    ///     // The user-specific task has now been processed
    /// }
    /// ```
    async fn process_user_specific_task(
        pg: &PostgresDatabase,
        task: &TaskMessage,
    ) -> TaskResult<()> {
        let action = &task.task_action;
        if action.eq("create_user") {
            let task: UserCreateTask = serde_json::from_str(&task.task_message).unwrap();
            task.process(pg).await;
        }
        Ok(())
    }
}