use bb8_postgres::tokio_postgres::{types::ToSql, Error};
use crossbeam_channel::{unbounded, Receiver, Sender};
use nanoid::nanoid;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::app::database::{postgres::PostgresDatabase, redis::RedisDatabase};

// sender

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

    fn generate_sender_specific_id(sender_type: SenderType) -> String {
        format!("{}-{}", sender_type.to_string(), nanoid!())
    }
}

// worker

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
    pub fn with_databases(pg: PostgresDatabase, redis: RedisDatabase) -> WorkerManager {
        WorkerManager { pg, redis }
    }

    /// Starts a listener for incoming messages on the global task channel.
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

    // Sends a message to global task channel.
    pub fn send(&self, sender_message: SenderMessage) {
        TASK_CHANNEL.0.send(sender_message).unwrap();
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
