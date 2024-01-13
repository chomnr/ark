use thiserror::Error;

pub type CacheResult<T> = Result<T, CacheError>;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Failed to write value into cache because the requested id already exists.")]
    CacheWriteFailure,
    #[error("Failed to update value from cache because the requested id does not exist.")]
    CacheUpdateFailure,
    #[error("Failed to delete value from cache because the requested id does not exist.")]
    CacheDeleteFailure,
    #[error("Failed to read value from cache because the requested id does not exist.")]
    CacheReadFailure
}