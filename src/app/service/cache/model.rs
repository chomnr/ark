use axum::async_trait;

use super::error::CacheResult;

#[async_trait]
pub trait Cacheable<T> {
    /// Adds an item to the cache.
    ///
    /// # Arguments
    /// * `value` - The item of type `T` to be added to the cache.
    ///
    /// # Returns
    /// `CacheResult<bool>` indicating the success (`true`) or failure (`false`) of the write operation.
    ///
    /// # Examples
    /// ```
    /// let item = ...; // An instance of type `T`
    /// let result = LocalCache::write(item).await;
    /// ```
    fn write(value: T) -> CacheResult<bool>;

    /// Updates an existing item in the cache.
    ///
    /// # Arguments
    /// * `value` - The item of type `T` with updated information.
    ///
    /// # Returns
    /// `CacheResult<bool>` indicating the success or failure of the update operation.
    ///
    /// # Examples
    /// ```
    /// let updated_item = ...; // An updated instance of type `T`
    /// let result = LocalCache::update(updated_item).await;
    /// ```
    fn update(value: T) -> CacheResult<bool>;

    /// Deletes an item from the cache.
    ///
    /// # Arguments
    /// * `value` - The item of type `T` to be removed.
    ///
    /// # Returns
    /// `CacheResult<bool>` indicating whether the deletion was successful.
    ///
    /// # Examples
    /// ```
    /// let item_to_delete = ...; // An instance of type `T` to delete
    /// let result = LocalCache::delete(item_to_delete).await;
    /// ```
    fn delete(value: T) -> CacheResult<bool>;

    /// Reads an item from the cache based on a lookup key.
    ///
    /// # Arguments
    /// * `value` - The lookup key of type `L` for retrieving the item.
    ///
    /// # Returns
    /// `CacheResult<bool>` indicating whether the item was found.
    ///
    /// # Examples
    /// ```
    /// let lookup_key = ...; // A lookup key of type `L`
    /// let result = LocalCache::read(lookup_key).await;
    /// ```
    fn read<L>(value: L) -> CacheResult<bool>;
}

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