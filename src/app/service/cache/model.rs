use super::{Cacheable, error::CacheResult};

/// Generic struct for caching objects of types that implement `Cacheable`.
///
/// `Cache` is designed to work with any type `T` that satisfies the `Cacheable` trait, enabling flexible and type-safe caching mechanisms.
///
/// # Type Parameters
/// - `T`: The type of object being cached. Must implement the `Cacheable` trait.
///
/// # Fields
/// - `cache`: An instance of the type `T` that represents the actual cache storage or mechanism.
///
/// # Examples
///
/// Creating a cache for a specific type:
///
/// ```
/// struct MyCacheableType;
/// impl Cacheable<MyCacheableType> for MyCacheableType {
///     // Implementation details
/// }
///
/// let my_cache = Cache { cache: MyCacheableType };
/// ```
///
/// This example demonstrates how to create a `Cache` instance for a custom type that implements the `Cacheable` trait.
pub struct Cache<T>
where
    T: Cacheable<T>,
{
    cache: T,
}

impl<T> Cache<T> where T: Cacheable<T> {
    pub fn write(value: T) -> CacheResult<bool> {
        T::write(value)
    }
    pub fn update(value: T) -> CacheResult<bool> {
        T::update(value)
    }
    pub fn delete(value: T) -> CacheResult<bool> {
        T::delete(value)
    }
    pub fn read<L>(lookup: L) -> CacheResult<bool> {
        T::read::<L>(lookup)
    }
}