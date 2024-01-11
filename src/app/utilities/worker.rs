use bb8_postgres::tokio_postgres::{types::ToSql, Error};
use crossbeam_channel::{unbounded, Receiver, Sender};
use nanoid::nanoid;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::app::database::{postgres::PostgresDatabase, redis::RedisDatabase};

#[derive(Clone, Copy)]
pub enum SenderType {
    Permission,
    Role,
    User,
}

impl SenderType {
    pub fn to_string(&self) -> String {
        match self {
            SenderType::Permission => return String::from("permission_req"),
            SenderType::Role => return String::from("role_req"),
            SenderType::User => return String::from("user_req"),
        }
    }
}

pub struct SenderMessage {
    sender_id: String,
    sender_type: SenderType,
    sender_message: String,
}

impl SenderMessage {
    /// Composes a `SenderMessage` from the provided `sender_type` and `sender_message`.
    ///
    /// # Arguments
    ///
    /// * `sender_type` - The type of the sender.
    /// * `sender_message` - The message to be sent, of type `T`.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `SenderMessage`.
    ///
    /// # Panics
    ///
    /// Panics if serialization of the `sender_message` fails.
    pub fn compose<T: for<'a> Deserialize<'a> + Serialize>(
        sender_type: SenderType,
        sender_message: T,
    ) -> SenderMessage {
        SenderMessage {
            sender_id: Self::generate_sender_specific_id(sender_type),
            sender_type,
            sender_message: serde_json::to_string(&sender_message).unwrap(),
        }
    }

    /// Generates a sender-specific ID based on the given `sender_type`.
    ///
    /// # Arguments
    ///
    /// * `sender_type` - The type of the sender for which the ID is being generated.
    ///
    /// # Returns
    ///
    /// Returns a `String` representing the unique sender-specific ID.
    fn generate_sender_specific_id(sender_type: SenderType) -> String {
        format!("{}-{}", sender_type.to_string(), nanoid!())
    }
}

#[derive(PartialEq, Eq)]
pub enum WorkerType {
    Sender,
    Receiver,
}

static TASK_CHANNEL: Lazy<(Sender<SenderMessage>, Receiver<SenderMessage>)> =
    Lazy::new(|| unbounded());

pub struct WorkerManager {
    pg: PostgresDatabase,
    redis: RedisDatabase,
}

impl WorkerManager {
    /// Creates a new instance of `WorkerManager` with specified PostgreSQL and Redis database connections.
    ///
    /// # Arguments
    ///
    /// * `pg` - A `PostgresDatabase` connection to be used by the `WorkerManager`.
    /// * `redis` - A `RedisDatabase` connection to be used by the `WorkerManager`.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `WorkerManager` configured with the specified database connections.
    pub fn with_databases(pg: PostgresDatabase, redis: RedisDatabase) -> WorkerManager {
        WorkerManager { pg, redis }
    }

    /// Starts a listener for incoming messages on the global task channel.
    ///
    /// # Examples
    ///
    /// ```
    /// let worker = WorkerManager::new();
    /// worker.listen(); // Start listening for messages
    /// ```
    pub fn listen(&self) {
        task::spawn(async move {
            println!("[ARC] worker initialized, now listening to requests.");
            for message in TASK_CHANNEL.1.iter() {
                match message.sender_type {
                    SenderType::Permission => {
                        println!("received a permission message {}", message.sender_id)
                        // process and cache (local cache)
                    }
                    SenderType::Role => {
                        println!("received a role message {}", message.sender_id)
                        // process and cache (local cache)
                    }
                    SenderType::User => {
                        println!("received a user message {}", message.sender_id)
                        // process user and cache (redis)
                    }
                }
            }
        });
    }

    /// Sends a `SenderMessage` to the global task channel.
    ///
    /// # Arguments
    ///
    /// * `sender_message` - The `SenderMessage` instance to be sent to the task channel.
    ///
    /// # Examples
    ///
    /// ```
    /// let worker_manager = WorkerManager::new();
    /// let message = SenderMessage { /* fields */ };
    /// worker_manager.send(message);
    /// ```
    pub fn send(&self, sender_message: SenderMessage) {
        TASK_CHANNEL.0.send(sender_message).unwrap();
    }

    /// Asynchronously processes a SQL query with the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `query` - A string slice containing the SQL query to be executed.
    /// * `params` - A slice of references to objects that implement `ToSql` and `Sync`.
    ///   These are the parameters to be bound to the SQL query.
    ///
    /// # Examples
    ///
    /// ```
    /// async fn example_usage(worker: &WorkerManager) {
    ///     let query = "INSERT INTO my_table (col1, col2) VALUES ($1, $2)";
    ///     let params: &[&(dyn ToSql + Sync)] = &[&"value1", &123];
    ///     match worker.process_query(query, params).await {
    ///         Ok(rows) => println!("{} rows inserted", rows),
    ///         Err(e) => println!("Error executing query: {}", e),
    ///     }
    /// }
    /// ```
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
