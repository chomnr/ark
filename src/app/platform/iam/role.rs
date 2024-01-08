use bb8_postgres::tokio_postgres::Error;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::cache::{CacheError, CacheResult, Cacheable},
};

pub static ROLE_CACHE: Lazy<DashMap<i32, Role>> = Lazy::new(|| DashMap::new());

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        ROLE_CACHE.insert(value.id, value.clone()).map_or_else(
            || {
                println!("[ARC] wrote to 'roles' cache [{}:{}]", value.id, value.name);
                Ok(true)
            },
            |_| Err(CacheError::CacheWriteFailure),
        )
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
                println!(
                    "[ARC] updated 'roles' cache [{}:{}] ==> [{}:{}]",
                    entry.id, entry.name, value.id, value.name
                );
                *entry = value.clone();
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
        ROLE_CACHE.remove(&value.id).map_or_else(
            || Err(CacheError::CacheDeleteFailure),
            |_| {
                RoleCache::delete(value.clone()).unwrap();
                println!("[ARC] deleted from 'roles' cache [{}]", value.id);
                Ok(true)
            },
        )
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
            .ok_or({
                println!(
                    "[ARC] MISSED retrieving role from 'roles' cache [{}]",
                    value.id
                );
                CacheError::CacheReadFailure
            })
    }
}

/// Represents a repository for managing `Role` entities using a PostgreSQL database.
///
/// This struct is a part of the data layer in the application architecture.
/// It encapsulates the connection and operations with a PostgreSQL database,
/// providing an abstraction layer for working with `Role` data.
///
/// The `pg` field holds a `PostgresDatabase` instance, which is responsible
/// for establishing and managing the database connection and executing SQL queries.
///
/// Use `RoleRepo` to perform CRUD (Create, Read, Update, Delete) operations
/// specific to `Role` entities within the PostgreSQL database.
pub struct RoleRepo {
    pg: PostgresDatabase,
}

impl RoleRepo {
    pub async fn preload_cache(&self) {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool.prepare("SELECT * FROM roles").await.unwrap();
        let rows = pool.query(&pstmt, &[]).await.unwrap();
        for row in rows {
            let role = Role {
                id: row.get("id"),
                name: row.get("role_name"),
            };
            RoleCache::write(role).unwrap();
        }
        println!(
            "[ARC] preloaded 'roles' cache with {} entries",
            ROLE_CACHE.len()
        )
    }
}

impl RoleRepo {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    /// Asynchronously creates a new role in the database.
    ///
    /// This function inserts a new role into the `roles` table. It uses the provided `role` object
    /// to get the role's name and inserts it along with a generated `id`.
    ///
    /// # Arguments
    /// * `role` - A `Role` object containing the details of the role to be created.
    ///
    /// # Returns
    /// A `Result` containing either the number of rows affected (usually 1 for a successful insert),
    /// or an `Error` in case of any issues during the insert operation.
    pub async fn create_role(&self, role: Role) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("INSERT INTO roles (role_name) VALUES($1) RETURNING id")
            .await
            .unwrap();
        match pool.query_one(&pstmt, &[&role.name]).await {
            Ok(row) => {
                let id: i32 = row.get(0);
                RoleCache::write(Role::builder().id(id).name(&role.name).build()).unwrap();
                Ok(1)
            }
            Err(er) => Err(er),
        }
    }

    /// Asynchronously updates an existing role in the database.
    ///
    /// This function updates the `role_name` of a role identified by `id` in the `roles` table.
    /// The `role` object provides the new name and the `id` of the role to update.
    ///
    /// # Arguments
    /// * `role` - A `Role` object containing the new role name and the `id` of the role to be updated.
    ///
    /// # Returns
    /// A `Result` containing either the number of rows affected (usually 1 for a successful update),
    /// or an `Error` in case of any issues during the update operation.
    pub async fn update_role(&self, role: Role) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("UPDATE roles SET role_name = '$1' WHERE id = $2;")
            .await
            .unwrap();
        //PostgresWorker::queue(HIGHPRIORIOTY, query, ddd)
        //PostgresWorker::queue()
        //RoleWorker::task().queue("sql", "the parameters")
        match pool.execute(&pstmt, &[&role.id, &role.name]).await {
            Ok(res) => {
                RoleCache::update(role).unwrap();
                Ok(res)
            }
            Err(er) => Err(er),
        }
    }

    /// Asynchronously deletes a role from the database.
    ///
    /// This function removes a role from the `roles` table based on the `id` provided in the `role` object.
    ///
    /// # Arguments
    /// * `role` - A `Role` object containing the `id` of the role to be deleted.
    ///
    /// # Returns
    /// A `Result` containing either the number of rows affected (usually 1 for a successful delete),
    /// or an `Error` in case of any issues during the delete operation.
    pub async fn delete_role(&self, role: Role) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("DELETE FROM roles WHERE id = $1;")
            .await
            .unwrap();
        match pool.execute(&pstmt, &[&role.id]).await {
            Ok(res) => {
                RoleCache::delete(role).unwrap();
                Ok(res)
            }
            Err(er) => Err(er),
        }
    }

    /// Asynchronously reads a role's details from the database.
    ///
    /// This function retrieves the details of a role from the `roles` table using the `id` provided in the `role` object.
    /// Note: The implementation is currently incomplete (`todo!()`).
    ///
    /// # Arguments
    /// * `role` - A `Role` object containing the `id` of the role to be read.
    ///
    /// # Returns
    /// A `Result` containing either the `Role` object with the retrieved details,
    /// or an `Error` in case of any issues during the read operation.
    pub async fn read_role(&self, role: Role) -> Result<Role, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let pstmt = pool
            .prepare("DELETE FROM roles WHERE id = $1;")
            .await
            .unwrap();
        pool.execute(&pstmt, &[&role.id]).await.unwrap();
        todo!()
    }
}

/*
// message here
pub struct RoleWorker;
pub struct RoleSenderMessage;

#[async_trait]
impl Worker for RoleWorker {
    async fn attach_pg_db() {
        todo!()
    }

    async fn listen() {
        let (tx, mut rx) = mpsc::channel(1000);
        todo!()
    }
}
*/
