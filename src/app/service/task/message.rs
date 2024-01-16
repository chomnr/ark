use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use super::error::{TaskError, TaskResult};

/// Represents the status of a task.
///
/// This enum is used to indicate whether a task has been completed successfully
/// or has failed. It's a part of the task response to signify the outcome of the task.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Indicates that the task was completed successfully.
    Completed,

    /// Indicates that the task has failed.
    Failed,
}


/// Represents the type of task.
/// 
/// This enum is used to identify what type of task is being sent to the
/// INBOUND channel. It ensures that the right handler is used.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TaskType {
    Permission
}

/// A request structure for a task.
///
/// This struct is used to encapsulate the details of a task request. It includes
/// the unique identifier of the task and the payload which contains the details
/// or data required to perform the task.
#[derive(Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    /// A unique identifier for the task.
    pub task_id: String,

    /// The payload of the task, containing the details or data for the task.
    pub task_payload: String,

    /// The type of task.
    pub task_type: TaskType
}

impl TaskRequest {
    /// Composes a new task request with a given payload.
    pub fn compose_request<T: for<'a> Deserialize<'a> + Serialize>(task_payload: T, task_type: TaskType) -> Self {
        Self {
            task_id: format!("task-{}", nanoid!(7)),
            task_payload: serde_json::to_string(&task_payload).unwrap(),
            task_type
        }
    }

    pub fn intepret_request_payload<T: for<'a> Deserialize<'a>>(
        task_request: &TaskRequest,
    ) -> TaskResult<T> {
        match serde_json::from_str::<T>(&task_request.task_payload) {
            Ok(result) => Ok(result),
            Err(_) => Err(TaskError::FailedToInterpretPayload),
        }
    }
}

/// A generic response structure for a task.
///
/// This struct is used to represent the response of a task. It is generic over `T`,
/// allowing for flexibility in the type of result that the task returns. It includes
/// the unique task identifier, the result of the task, and the status of the task.
#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
    /// The unique identifier of the task.
    pub task_id: String,

    /// The result of the task. The type of this field is a String to accommodate
    /// various types of task results.
    pub task_result: String,

    /// The status of the task, indicating whether it was completed successfully
    /// or failed.
    pub task_status: TaskStatus,

    /// The errors that occur when the task_status fails when processing the given
    /// task.
    pub task_error: Vec<String>,
}

impl TaskResponse {
    /// Composes a TaskResponse from a TaskRequest sparingly.
    pub fn compose_response<'a, T: Deserialize<'a> + Serialize>(
        request: TaskRequest,
        task_status: TaskStatus,
        task_result: T,
        task_error: Vec<String>,
    ) -> Self {
        Self {
            task_id: request.task_id,
            task_result: serde_json::to_string(&task_result).unwrap(),
            task_status,
            task_error,
        }
    }

    pub fn throw_failed_response(request: TaskRequest, errors: Vec<String>) -> Self {
        Self {
            task_id: request.task_id,
            task_result: String::default(),
            task_status: TaskStatus::Failed,
            task_error: errors,
        }
    }
}
