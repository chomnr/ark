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

use self::message::{TaskRequest, TaskResponse};

pub mod error;
pub mod message;

/// Receives tasks and processing them.
static INBOUND: Lazy<(Sender<TaskRequest>, Receiver<TaskRequest>)> = Lazy::new(|| unbounded());

/// Receives results from tasks and sends them back to the origin point.
static OUTBOUND: Lazy<(Sender<TaskResponse>, Receiver<TaskResponse>)> = Lazy::new(|| unbounded());