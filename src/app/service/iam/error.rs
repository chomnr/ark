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
    PermissionParameterMismatch,
    PermissionCreationFailed,
    PermissionDeletionFailed,
    PermissionFailedToLinkToRole,
    PermissionFailedToDeleteLinkToRole,
}

impl fmt::Display for IamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IamError::PermissionParameterMismatch => write!(f, "Parameter mismatch: provided values don't match expected count for this action."),
            IamError::PermissionCreationFailed => write!(f, "Failed to create permission: permission with the given key or name already exists."),
            IamError::PermissionDeletionFailed => write!(f, "Failed to delete permission: permission with the given key does not exists."),
            IamError::PermissionFailedToLinkToRole => write!(f, "Failed to link role and permission: either role or permission does not exist."),
            IamError::PermissionFailedToDeleteLinkToRole => write!(f, "Failed to unlink role and permission: either role or permission does not exist."),
        }
    }
}

impl IntoResponse for IamError {
    fn into_response(self) -> axum_core::response::Response {
        match self {
            IamError::PermissionParameterMismatch => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "PermissionParameterMismatch", "The provided parameters do not align with the expected parameters for the specified PermissionAction.")),
            ).into_response(),
            IamError::PermissionCreationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "PermissionCreationFailed", "The system encountered an issue while attempting to create the specified permission.")),
            ).into_response(),
            IamError::PermissionDeletionFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "PermissionDeletionFailed", "The system encountered an issue while attempting to delete the specified permission.")),
            ).into_response(),
            IamError::PermissionFailedToLinkToRole => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "PermissionFailedToLinkToRole", "Unable to establish a connection between the specified role and permission. Either the role or permission does not exist in the system.")),
            ).into_response(),
            IamError::PermissionFailedToDeleteLinkToRole => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "PermissionFailedToDeleteLinkToRole", "The system is unable to remove the association between the specified role and permission, as either the role or permission does not exist.")),
            ).into_response(),
        }
    }
}
