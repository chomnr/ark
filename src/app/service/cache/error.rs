use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("CacheFieldNotFound")]
    CacheFieldNotFound,
    #[error("CacheItemNotFound")]
    CacheItemNotFound,
    #[error("CacheInternalError")]
    CacheInternalError,
    #[error("CacheUniqueViolation")]
    CacheUniqueViolation
}

pub type CacheResult<T> = Result<T, CacheError>;
