use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::app::service::cache::{Cacheable, CacheResult, CacheError};

static ROLE_CACHE: Lazy<DashMap<i32, Role>> = Lazy::new(|| DashMap::new());

/// Represents a role within the application.
///
/// `Role` is a struct that defines the properties and characteristics of a user role.
///
/// # Fields
/// - `id`: An `i32` representing the unique identifier of the role.
/// - `name`: A `String` that holds the name of the role.
///
/// Additional methods can be implemented for `Role` to provide functionalities such 
/// as creating a new role, updating its properties, or other role-related operations.
#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

impl Role {
    pub fn new(id: i32, role_name: &str) -> Self {
        Self {
            id,
            name: String::from(role_name),
        }
    }

    pub fn builder() -> RoleBuilder {
        RoleBuilder::new()
    }
}

/// Builder for creating instances of `Role`.
///
/// `RoleBuilder` facilitates the construction of `Role` objects, allowing for step-by-step 
/// setting of its properties.
///
/// # Fields
/// - `id`: An `i32` representing the unique identifier of the role.
/// - `name`: A `String` for the name of the role.
///
/// This builder pattern is particularly useful for creating `Role` objects with optional or 
/// complex initialization logic.
pub struct RoleBuilder {
    pub id: i32,
    pub name: String,
}

impl Default for RoleBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

impl RoleBuilder {
    fn new() -> Self {
        Self {
            id: 0,
            name: String::default(),
        }
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    pub fn build(&self) -> Role {
        Role {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

/// Implementation of the `Cacheable` trait for `RoleCache`.
///
/// This implementation provides caching functionalities tailored specifically for `Role` 
/// objects.
///
/// By implementing `Cacheable`, `RoleCache` can perform operations such as write, update, 
/// delete, and read, 
/// which are essential for managing `Role` instances in a cache.
///
/// The exact details of how these operations interact with the underlying caching 
/// mechanism (presumably within `RoleCache`) would be defined within each method's 
/// implementation.
pub struct RoleCache;

impl Cacheable<Role> for RoleCache {
    /// Writes a `Role` to the `ROLE_CACHE`.
    ///
    /// Inserts a `Role` into the cache. If the insertion is successful, it returns `Ok(true)`.
    /// If there is a failure in writing to the cache, it returns an `Err` with `CacheError::CacheWriteFailure`.
    ///
    /// # Arguments
    /// * `value` - The `Role` to be inserted into the cache.
    ///
    /// # Returns
    /// `CacheResult<bool>` indicating success or failure of the operation.
    fn write(value: Role) -> CacheResult<bool> {
        ROLE_CACHE
            .insert(value.id, value)
            .map_or_else(|| Ok(true), |_| Err(CacheError::CacheWriteFailure))
    }

    /// Updates an existing `Role` in the `ROLE_CACHE`.
    ///
    /// Looks for the `Role` in the cache by its ID and updates it if found. Returns `Ok(true)` if successful.
    /// If the `Role` is not found, it returns an `Err` with `CacheError::CacheUpdateFailure`.
    ///
    /// # Arguments
    /// * `value` - The `Role` to be updated in the cache.
    ///
    /// # Returns
    /// `CacheResult<bool>` indicating success or failure of the operation.
    fn update(value: Role) -> CacheResult<bool> {
        ROLE_CACHE
            .get_mut(&value.id)
            .map(|mut entry| {
                *entry = value;
                true
            })
            .ok_or(CacheError::CacheUpdateFailure)
    }

    /// Deletes a `Role` from the `ROLE_CACHE`.
    ///
    /// Removes the `Role` from the cache using its ID. If the removal is successful, it returns `Ok(true)`.
    /// If the `Role` is not found in the cache, it returns an `Err` with `CacheError::CacheDeleteFailure`.
    ///
    /// # Arguments
    /// * `value` - The `Role` to be deleted from the cache.
    ///
    /// # Returns
    /// `CacheResult<bool>` indicating success or failure of the operation.
    fn delete(value: Role) -> CacheResult<bool> {
        ROLE_CACHE
            .remove(&value.id)
            .map_or_else(|| Err(CacheError::CacheDeleteFailure), |_| Ok(true))
    }

    /// Reads a `Role` from the `ROLE_CACHE`.
    ///
    /// Retrieves the `Role` from the cache using its ID. If found, it returns `Ok(Role)`.
    /// If the `Role` is not found, it returns an `Err` with `CacheError::CacheDeleteFailure`.
    ///
    /// # Arguments
    /// * `value` - The `Role` to be read from the cache.
    ///
    /// # Returns
    /// `CacheResult<Role>` indicating success or failure of the operation.
    fn read(value: Role) -> CacheResult<Role> {
        ROLE_CACHE
            .get(&value.id)
            .map(|v| Role::new(v.id, &v.name))
            .ok_or(CacheError::CacheDeleteFailure)
    }
}
