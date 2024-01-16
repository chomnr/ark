//
// Task System
//
// +---------------------+
// | User Sends Request  |
// +----------+----------+
//            |
//            v
// +----------+----------+       +--------------------------+
// | Inbound Channel     |       | Process Request          |
// | Receives Request    | ----> | and Check for Errors     |
// +---------------------+       +----------+---------------+
//                                          |
//                               +----------+-----------+
//                               |                      |
//                       +-------v------+       +-------v------+
//                       | Process      |       | Handle       |
//                       | Successfully |       | Errors       |
//                       +-------+------+       +-------+------+
//                               |                      |
//                               v                      v
//                       +-------+------+       +-------+------+
//                       | Outbound     |       | Outbound     |
//                       | Channel      |       | Channel      |
//                       | Sends Result |       | Sends Result |
//                       | to User      |       | to User      |
//                       +--------------+       +--------------+
// 

use axum::async_trait;
use crossbeam_channel::{Receiver, Sender, unbounded};
use once_cell::sync::Lazy;

use crate::app::database::postgres::PostgresDatabase;

use self::message::{TaskRequest, TaskResponse};

pub mod error;
pub mod manager;
pub mod message;

/// Receives tasks and processing them.
static INBOUND: Lazy<(Sender<TaskRequest>, Receiver<TaskRequest>)> = Lazy::new(|| unbounded());

/// Receives results from tasks and sends them back to the origin point.
static OUTBOUND: Lazy<(Sender<TaskResponse>, Receiver<TaskResponse>)> = Lazy::new(|| unbounded());

/// Used to create specific tasks.
#[async_trait]
pub trait Task<P, D> {
    async fn run(db: &D, request: P) -> TaskResponse;
}

/// Handles the task
#[async_trait]
pub trait TaskHandler {
    async fn handle(pg: &PostgresDatabase, task_request: TaskRequest) -> TaskResponse;
}
