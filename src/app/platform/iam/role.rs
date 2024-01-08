use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::app::platform::cache::{Cacheable, CacheResult, CacheError};

static ROLE_CACHE: Lazy<DashMap<i32, Role>> = Lazy::new(|| DashMap::new());

/// A representation of a user role in the system.
///
/// This struct is used to define and handle properties associated with a user role.
///
/// # Fields
/// - `id`: An integer representing the unique identifier of the role.
/// - `name`: A string holding the name of the role.
///
/// # Examples
///
/// Creating a new `Role`:
///
/// ```
/// let admin_role = Role::new(1, "Administrator");
/// ```
///
/// Using a builder to create a `Role`:
///
/// ```
/// let role = Role::builder()
///     .id(2)
///     .name("Editor")
///     .build();
/// ```
///
/// # Methods
///
/// - `new`: Constructs a new `Role`.
/// - `builder`: Provides a `RoleBuilder` for building a `Role` instance.
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

impl Cacheable<Role> for Role {
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

/// Builder for constructing a `Role` instance.
///
/// Allows for customizable creation of `Role` objects. Each field can be set independently before finalizing the construction.
///
/// # Fields
/// - `id`: An integer representing the role's unique identifier.
/// - `name`: A string specifying the name of the role.
///
/// # Methods
///
/// - `default`: Creates a new `RoleBuilder` with default values for `id` and `name`.
/// - `id` (deprecated): Sets the `id` of the role. This method is deprecated and may be removed in future versions.
/// - `name`: Sets the `name` of the role.
/// - `build`: Finalizes and builds a `Role` instance based on the set values.
///
/// The `build` method clones the `name` field to create a new `Role`, ensuring the builder can be reused or modified later without affecting the created `Role`.
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