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
    #[error("PermissionFieldNotFound")]
    PermissionFieldNotFound,
    #[error("PermissionNotFound")]
    PermissionNotFound,
    #[error("PermissionFailedToPreload")]
    PermissionFailedToPreload,
    #[error("PermissionLinkAlreadyExist")]
    PermissionLinkAlreadyExist,
    #[error("RoleDuplication")]
    RoleDuplication,
    #[error("RoleFieldNotFound")]
    RoleFieldNotFound,
    #[error("RoleNotFound")]
    RoleNotFound,
    #[error("RoleFailedToPreload")]
    RoleFailedToPreload,
    #[error("RoleLinkFailedToLink")]
    RoleLinkFailedToLink,
    // Task
    #[error("TaskInternalError")]
    TaskInternalError,
    #[error("FieldNotMutable")]
    FieldNotMutable,
    // User
    #[error("UserNotFound")]
    UserNotFound
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
