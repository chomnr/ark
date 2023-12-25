use core::fmt;

pub type AccountRepositoryResult<T> = Result<T, AccountRepositoryError>;

#[derive(Debug)]
pub enum AccountRepositoryError {
    FieldMismatch,
    FailedToCreateIdentity
}

static PREFIX: &str = "[ARC] error: ";

impl fmt::Display for AccountRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountRepositoryError::FieldMismatch => write!(f, "{} A field has an incompatiable method binded with it.", PREFIX),
            AccountRepositoryError::FailedToCreateIdentity => write!(f, "{} Failed to create an identity for an individual.", PREFIX),
        }
    }
}