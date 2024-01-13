use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tokio::task;

use crate::app::database::{postgres::PostgresDatabase, redis::RedisDatabase};

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
            for task in TASK_CHANNEL.1.iter() {
                Self::process_task(&pg_clone, task);
            }
        });
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
    fn process_task(pg: &PostgresDatabase, task: TaskMessage) {
        match task.task_type {
            super::model::TaskType::Permission => {
                match Self::process_permission_specific_task(pg, task) {
                    Ok(_) => todo!(), /* Send to receiver with the necessary parameters saying it was a success */
                    Err(_) => todo!(), /* Sends to receiver saying it failed... */
                }
            },
            super::model::TaskType::Role => {
                match Self::process_role_specific_task(pg, task) {
                    Ok(_) => todo!(), /* Send to receiver with the necessary parameters saying it was a success */
                    Err(_) => todo!(), /* Sends to receiver saying it failed... */
                }
            }
            super::model::TaskType::User => {
                match Self::process_user_specific_task(pg, task) {
                    Ok(_) => todo!(), /* Send to receiver with the necessary parameters saying it was a success */
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
    ///
    /// # Note
    /// The function currently uses a `todo!()` macro as a placeholder. This should be replaced with the
    /// actual implementation that handles the task based on its permission requirements.
    fn process_permission_specific_task(pg: &PostgresDatabase, task: TaskMessage) -> TaskResult<()> {
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
    fn process_role_specific_task(pg: &PostgresDatabase, task: TaskMessage) -> TaskResult<()> {
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
    fn process_user_specific_task(pg: &PostgresDatabase, task: TaskMessage) -> TaskResult<()> {
        todo!()
    }
}
