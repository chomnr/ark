use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("CacheFieldNotFound")]
    FieldNotFound,
    #[error("CacheItemNotFound")]
    ItemNotFound,
    #[error("CacheInternalError")]
    InternalError,
    #[error("CacheUniqueViolation")]
    UniqueViolation,
    #[error("CacheFailedToInterpretPayload")]
    FailedToInterpretPayload
}

pub type CacheResult<T> = Result<T, CacheError>;
