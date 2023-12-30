use core::fmt;

pub type IamResult<T> = Result<T, IamError>;

#[derive(Debug)]
pub enum IamError {
    PermissionParameterMismatch,
    UnableToCreatePermission,
    UnableToDeletePermission,
    UnableToLinkRoleToPermission,
    UnableToDeleteRoleToPermission
}

impl fmt::Display for IamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IamError::PermissionParameterMismatch => write!(f, "permission parameters do not match the permissionaction parameters."),
            IamError::UnableToCreatePermission => write!(f, "unable to create permission."),
            IamError::UnableToDeletePermission => write!(f, "unable to delete permission."),
            IamError::UnableToLinkRoleToPermission => write!(f, "unable to link role_permission. (role or permission does not exist)."),
            IamError::UnableToDeleteRoleToPermission => write!(f, "unable to delete role_permission. (role or permission does not exist)"),
        }
    }
}