use bb8_postgres::tokio_postgres::{types::ToSql, Error};
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tokio::task;

use crate::app::{
    database::{postgres::PostgresDatabase, redis::RedisDatabase},
    services::task::model::TaskType,
};

use super::model::TaskMessage;

static TASK_CHANNEL: Lazy<(Sender<TaskMessage>, Receiver<TaskMessage>)> = Lazy::new(|| unbounded());

pub struct TaskManager {
    pg: PostgresDatabase,
    redis: RedisDatabase,
}

impl TaskManager {
    /// Creates a new instance of the containing struct with specified PostgreSQL and Redis database connections.
    ///
    /// # Arguments
    /// * `pg` - A `PostgresDatabase` connection.
    /// * `redis` - A `RedisDatabase` connection.
    ///
    /// # Returns
    /// Returns an instance of the struct initialized with the provided database connections.
    ///
    /// # Example
    /// ```
    /// let postgres = PostgresDatabase::new(/* config */);
    /// let redis = RedisDatabase::new(/* config */);
    /// let task_mgr = TaskManager::with_databases(postgres, redis);
    /// ```
    pub fn with_databases(pg: PostgresDatabase, redis: RedisDatabase) -> Self {
        Self { pg, redis }
    }

    /// Listens for incoming task messages on a dedicated channel.
    ///
    /// # Examples
    /// ```
    /// let worker = WorkerManager::new(pg, redis); // Assuming WorkerManager contains the listen method
    /// worker.listen(); // Start listening for tasks
    /// ```
    pub fn listen(&self) {
        println!("[ARC] task initialized, now listening for incoming tasks.");
        task::spawn(async move {
            for message in TASK_CHANNEL.1.iter() {
                println!("{}", message.task_id);
                match message.task_type {
                    TaskType::Permission => {
                        // ...
                    }
                    TaskType::Role => {
                        // ...
                    }
                    TaskType::User => {

                        // ...
                    }
                }
            }
        });
    }

    /// Sends a `TaskMessage` to the global task channel.
    ///
    /// # Arguments
    /// * `task_message` - The `TaskMessage` to be sent.
    ///
    /// # Examples
    /// ```
    /// let worker = WorkerManager::new(); // Assuming WorkerManager contains the send method
    /// let message = TaskMessage { /* fields */ };
    /// worker.send(message); // Enqueue a task for processing
    /// ```
    pub fn send(task_message: TaskMessage) {
        TASK_CHANNEL.0.send(task_message).unwrap();
    }

    /// Asynchronously processes a SQL query with the provided parameters.
    async fn process_query(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool.prepare(query).await.unwrap();
        pool.execute(&stmt, params).await
    }
}
