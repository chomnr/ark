use thiserror::Error;

pub type IamResult<T> = Result<T, IamError>;

#[derive(Error, Debug)]
pub enum IamError {
    #[error("Failed to write role into database because the requested role already exists.")]
    RoleCreationFailure,
    #[error("Failed to delete role from cache because the requested id does not exist.")]
    RoleDeletionFailure,
}