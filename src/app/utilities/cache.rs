#![allow(private_bounds)]

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

/// Trait defining the interface for cacheable items in Rust.
///
/// `Cacheable` provides a set of methods to be implemented by objects that can be stored in a cache.
/// The methods are designed for managing the lifecycle of cache entries, including writing, updating,
/// deleting, and reading data.
///
/// To ensure flexibility and compatibility with various caching mechanisms, `Cacheable` methods do not
/// modify the object's state (`&self` instead of `&mut self`). This trait can be implemented by any type
/// that needs to be cached, allowing it to be used within a concurrent caching system like `DashMap`.
pub trait Cacheable<T> {
    fn write(value: T) -> CacheResult<bool>;
    fn update(value: T) -> CacheResult<bool>;
    fn delete(value: T) -> CacheResult<bool>;
    fn read(value: T) -> CacheResult<T>;
}

/// Enum representing the possible outcomes of a cache operation.
///
/// `CacheStatus` is used to indicate whether a requested item was found in the cache (a "hit") or
/// not (a "miss").
///
/// # Variants
/// - `Hit`: Indicates that the requested item was found in the cache.
/// - `Miss`: Signifies that the requested item was not found in the cache.
enum CacheStatus {
    Hit,
    Miss,
}

/*
/// Struct `Cache` is a concurrent cache implementation in Rust using DashMap.
///
/// This struct encapsulates a DashMap that maps strings to boxed objects implementing
/// the `Cacheable` trait. This allows for storing various types of values in a type-safe
/// way, as long as they implement `Cacheable`.
///
/// `DashMap` is used here as it is a concurrent associative array or hashmap. It is
/// designed to handle concurrency, making it suitable for multi-threaded environments.
/// DashMap offers an API similar to `std::collections::HashMap` but with modifications
/// to support concurrent access.
pub struct Cache;

impl Cache {
    pub fn write<T: Cacheable<T>>(value: T) -> CacheResult<bool> {
        T::write(value)
    }
    pub fn update<T: Cacheable<T>>(value: T) -> CacheResult<bool> {
        T::update(value)
    }
    pub fn delete<T: Cacheable<T>>(value: T) -> CacheResult<bool> {
        T::delete(value)
    }
    pub fn read<T: Cacheable<T>>(value: T) -> CacheResult<T> {
        // log result check if it was hit or msis.
        T::read(value)
    }
}
*/

pub fn test(){
    
}