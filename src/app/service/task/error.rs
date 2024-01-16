use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
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