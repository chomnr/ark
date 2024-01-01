use core::fmt;

use axum::{http::StatusCode, Json};
use axum_core::response::IntoResponse;
use serde::Serialize;

pub type IamResult<T> = Result<T, IamError>;

/// Represents an error response structure for web requests.
///
/// This struct is used to serialize error information into a JSON format when responding to HTTP requests.
///
/// Attributes:
/// - `code`: A `u16` HTTP status code representing the type of error.
/// - `error`: A `String` specifying a short error identifier or code.
/// - `message`: A detailed `String` message describing the error.
#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, error: &str, message: &str) -> Self {
        ErrorResponse {
            code: code.as_u16(),
            error: error.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum IamError {
    ParameterMismatch,
    PermissionCreationFailed,
    PermissionDeletionFailed,
    PermissionFailedToLinkToRole,
    PermissionFailedToDeleteLinkToRole,
    RoleCreationFailed,
    RoleDeletionFailed,
    UserCreationFailed
}

impl fmt::Display for IamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IamError::ParameterMismatch => write!(f, "Parameter mismatch: provided values don't match expected count for this action."),
            IamError::PermissionCreationFailed => write!(f, "Failed to create permission: permission with the given key or name already exists."),
            IamError::PermissionDeletionFailed => write!(f, "Failed to delete permission: permission with the given key does not exists."),
            IamError::PermissionFailedToLinkToRole => write!(f, "Failed to link role and permission: either role or permission does not exist."),
            IamError::PermissionFailedToDeleteLinkToRole => write!(f, "Failed to unlink role and permission: either role or permission does not exist."),
            IamError::RoleCreationFailed => write!(f, "Failed to create role: role with the given name already exists."),
            IamError::RoleDeletionFailed => write!(f, "Failed to delete role: role with the given name does not exists."),
            IamError::UserCreationFailed => write!(f, "Failed to create user: this should not be possible."),
        }
    }
}

impl IntoResponse for IamError {
    fn into_response(self) -> axum_core::response::Response {
        match self {
            IamError::ParameterMismatch => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "ParameterMismatch", "The provided parameters do not align with the expected parameters for the specified action.")),
            ).into_response(),
            IamError::PermissionCreationFailed => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::new(StatusCode::CONFLICT, "PermissionCreationFailed", "The system encountered an issue while attempting to create the specified permission.")),
            ).into_response(),
            IamError::PermissionDeletionFailed => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(StatusCode::NOT_FOUND, "PermissionDeletionFailed", "The system encountered an issue while attempting to delete the specified permission.")),
            ).into_response(),
            IamError::PermissionFailedToLinkToRole => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::new(StatusCode::CONFLICT, "PermissionFailedToLinkToRole", "Unable to establish a connection between the specified role and permission. Either the role or permission does not exist in the system.")),
            ).into_response(),
            IamError::PermissionFailedToDeleteLinkToRole => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(StatusCode::NOT_FOUND, "PermissionFailedToDeleteLinkToRole", "The system is unable to remove the association between the specified role and permission, as either the role or permission does not exist.")),
            ).into_response(),
            IamError::RoleCreationFailed => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::new(StatusCode::CONFLICT, "RoleCreationFailed", "The system encountered an issue while attempting to create the specified role.")),
            ).into_response(),
            IamError::RoleDeletionFailed => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::new(StatusCode::NOT_FOUND, "RoleDeletionFailed", "The system encountered an issue while attempting to delete the specified role.")),
            ).into_response(),
            IamError::UserCreationFailed => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::new(StatusCode::CONFLICT, "UserCreationFailed", "The system encountered an issue while attempting to create the specified user.")),
            ).into_response(),
        }
    }
}
