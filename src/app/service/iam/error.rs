use core::fmt;

pub type UserRepositoryResult<T> = Result<T, UserRepositoryError>;

#[derive(Debug)]
pub enum UserRepositoryError {
    FailedToCreateIdentity
}

static PREFIX: &str = "[ARC] error: ";

impl fmt::Display for UserRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRepositoryError::FailedToCreateIdentity => write!(f, "{} Failed to create an identity for an individual.", PREFIX),
        }
    }
}