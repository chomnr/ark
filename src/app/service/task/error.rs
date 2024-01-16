use bb8_postgres::tokio_postgres::error;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    // Internal
    #[error("FailedToInterpretPayload")]
    FailedToInterpretPayload,
    #[error("FailedToFindAction")]
    FailedToFindAction,
    #[error("FailedToCompleteTask")]
    FailedToCompleteTask,
    // Permission
    #[error("PermissionDuplication")]
    PermissionDuplication,
    #[error("PermissionNotFound")]
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
