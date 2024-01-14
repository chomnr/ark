use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

use self::model::{TaskMessageResult, TaskMessage};

pub mod error;
pub mod manager;
pub mod model;

// `TASK_CHANNEL` is a unbounded channel for sending and receiving `TaskMessage` objects.
static TASK_CHANNEL: Lazy<(Sender<TaskMessage>, Receiver<TaskMessage>)> = Lazy::new(|| unbounded());

// `TASK_RESULT_CHANNEL` is for sending and receiving `TaskMessageResult` objects.
static TASK_RESULT_CHANNEL: Lazy<(Sender<TaskMessageResult>, Receiver<TaskMessageResult>)> =
    Lazy::new(|| unbounded());
