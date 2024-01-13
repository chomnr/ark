use super::error::CacheResult;

/// Trait defining the interface for cacheable items in Rust.
///
/// `Cacheable` provides a set of methods to be implemented by objects that can be stored in a cache.
/// The methods are designed for managing the lifecycle of cache entries, including writing, updating,
/// deleting, and reading data.
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
pub enum CacheStatus {
    Hit,
    Miss,
}