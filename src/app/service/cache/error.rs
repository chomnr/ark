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
    FailedToInterpretPayload,
    #[error("CacheFailedToFindAction")]
    FailedToFindAction,
    #[error("FailedToCompleteCache")]
    FailedToCompleteCache,
    #[error("IdentifierMustBeAUuid")]
    IdentifierMustBeAUuid
}

pub type CacheResult<T> = Result<T, CacheError>;
