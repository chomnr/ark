use core::fmt;

pub type UserRepositoryResult<T> = Result<T, UserRepositoryError>;

#[derive(Debug)]
pub enum UserRepositoryError {
    FieldMismatch,
    FailedToCreateIdentity
}

static PREFIX: &str = "[ARC] error: ";

impl fmt::Display for UserRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRepositoryError::FieldMismatch => write!(f, "{} A field has an incompatiable method binded with it.", PREFIX),
            UserRepositoryError::FailedToCreateIdentity => write!(f, "{} Failed to create an identity for an individual.", PREFIX),
        }
    }
}