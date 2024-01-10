use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;
use tokio::task;

//use  ::{postgres::PostgresDatabase, redis::RedisDatabase};

/*
// Enum representing sender and receiver.
#[derive(PartialEq, Eq)]
pub enum WorkerChannelType {
    Sender,
    Receiver,
}

// Enum representing different types of workers.
#[derive(PartialEq, Eq)]
pub enum SenderType {
    User,
    Role,
    Permission,
}

// Struct for worker messages.
pub struct WorkerMessage {
    pub sender_type: SenderType,
    pub sender_message: String,
}

// Channels for task requests and results.
static TASK_CHANNEL: Lazy<(Sender<WorkerMessage>, Receiver<WorkerMessage>)> =
    Lazy::new(|| unbounded());
static RESULT_CHANNEL: Lazy<(Sender<WorkerMessage>, Receiver<WorkerMessage>)> =
    Lazy::new(|| unbounded());

// Struct representing a worker channel with sender and receiver.
pub struct WorkerChannel {
    pub sender: Sender<WorkerMessage>,
    pub receiver: Receiver<WorkerMessage>,
}

// WorkerManager responsible for managing workers.
pub struct WorkerManager {
    pg: PostgresDatabase,
    redis: RedisDatabase,
}
*/

/* 

impl WorkerManager {
    pub fn with_databases(pg: PostgresDatabase, redis: RedisDatabase) -> WorkerManager {
        WorkerManager { pg, redis }
    }

    fn process_task(sender_type: SenderType, sender_message: String) {
        todo!()
    }
    /*
    pub fn listen(channel_type: WorkerChannelType) {
        task::spawn(async move {
            match channel_type {
                WorkerChannelType::Sender => {
                    // Sender
                }
                WorkerChannelType::Receiver => {
                    // Receiver
                }
            }
        });
    }

    fn process_task(sender_type: SenderType, sender_message: String) {
        match sender_type {
            SenderType::User => {

            },
            SenderType::Role => {

            },
            SenderType::Permission => {

            },
        }
    }
    */
    /*
    pub fn send_task(worker_type: WorkerSubmissionType, worker_message: String) {
        let message = WorkerMessage {
            worker_type,
            worker_message,
        };
        TASK_CHANNEL.0.send(message).unwrap();
    }
    */
}

*/

/*
// Starts listening for new task requests.
pub fn start_task_listener(&self) {
    task::spawn(async move {
        for message in Self::task_submission_channel().receiver.iter() {
            // get worker
            // get type then deserialize it.
        }
        // Logic to listen for new tasks
        todo!()
    });
}

// Starts listening for results of processed tasks.
pub fn start_result_listener(&self) {
    task::spawn(async move {
        // Logic to listen for task results
        todo!()
    });
}

// Provides a channel for submitting tasks.
fn task_submission_channel() -> WorkerChannel {
    WorkerChannel {
        sender: TASK_CHANNEL.0.clone(),
        receiver: TASK_CHANNEL.1.clone(),
    }
}

// Provides a channel for receiving task results.
fn task_result_channel() -> WorkerChannel {
    WorkerChannel {
        sender: RESULT_CHANNEL.0.clone(),
        receiver: RESULT_CHANNEL.1.clone(),
    }
}
*/
