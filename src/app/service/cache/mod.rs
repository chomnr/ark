use axum::async_trait;

use self::error::CacheResult;

pub mod error;
pub mod model;

#[async_trait]
pub(super) trait Cacheable<T> {
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
    fn read(value: T) -> CacheResult<T>;
}
