use bb8_postgres::tokio_postgres::error;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    // Internal
    #[error("Failed to interpret the payload. The operation cannot be completed because the payload format is unrecognized or malformed. Ensure that the payload is correctly formatted and adheres to the expected structure.")]
    FailedToInterpretPayload,
    #[error("Action not found. The requested action could not be located in the system. Ensure that the action identifier is correct and try again.")]
    FailedToFindAction,
    #[error("Task completion failed. The system was unable to successfully complete the requested task. Verify the task parameters and ensure all prerequisites are met before retrying.")]
    FailedToCompleteTask,
    // Permission
    #[error("Duplicate permission detected. The operation cannot be completed because the permission you are trying to add already exists. Ensure that each permission is unique.")]
    PermissionDuplication,
    #[error("Permission not found. The requested operation cannot be performed as the specified permission does not exist in the system. Please check the permission identifier and try again.")]
    PermissionNotFound,
}

pub type TaskResult<T> = Result<T, TaskError>;

impl Serialize for TaskError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
