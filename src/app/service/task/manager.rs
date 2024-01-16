use serde::Deserialize;

use crate::app::{
    database::postgres::PostgresDatabase,
    platform::iam::permission::task::PermissionTaskHandler,
    service::task::{
        message::{TaskStatus, TaskType},
        TaskHandler,
    },
};

use super::{
    error::{TaskError, TaskResult},
    message::{TaskRequest, TaskResponse},
    INBOUND, OUTBOUND,
};

/// A structure for handling tasks within the system.
///
/// This struct primarily interacts with a PostgreSQL database, represented by the `pg`
/// field. This struct is capable of sending tasks to be processed and listening for
/// tasks from various sources. It uses the PostgreSQL database for storing and retrieving
/// task-related information.
pub struct TaskManager {
    pg: PostgresDatabase,
}

impl TaskManager {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    pub fn listen(&self) {
        let pg_clone = self.pg.clone();
        tokio::task::spawn(async move {
            let inbound_receiver = &INBOUND.1;
            println!("[ARK] task initalized, now listening to incoming requests.");
            while let Ok(task_request) = inbound_receiver.recv() {
                println!(
                    "[ARK] successfully received a task from {} with type: {:?}",
                    task_request.task_id,
                    task_request.task_type.clone()
                );
                match task_request.task_type {
                    TaskType::Permission => {
                        let task_response = PermissionTaskHandler::handle(&pg_clone, task_request).await;
                        Self::send_response(task_response);
                    }
                }
            }
        });
    }

    pub fn send<T: for<'a> Deserialize<'a>>(task_request: TaskRequest) -> TaskResponse {
        INBOUND.0.send(task_request.clone()).unwrap();
        for task in OUTBOUND.1.iter() {
            if task.task_id.eq(&task_request.task_id) {
                if task.task_status.eq(&TaskStatus::Completed) {
                    println!("[ARK] {} has been completed", task.task_id);
                } else {
                    println!("[ARK] {} has failed", task.task_id);
                }
                return task;
            }
        }
        unreachable!()
    }

    pub fn send_response(task_response: TaskResponse) {
        OUTBOUND.0.send(task_response).unwrap();
    }
}
