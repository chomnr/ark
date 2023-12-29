use bb8_postgres::tokio_postgres::{Statement, Error, types::Kind};

use crate::app::database::postgres::PostgresDatabase;

/// Represents a permission entity with an ID, name, and key.
///
/// Fields:
/// - `id`: Integer identifier for the permission. (auto generated on insert)
/// - `name`: Human-readable name of the permission.
/// - `key`: Unique key associated with the permission.
///
/// Methods:
/// - `new`: Constructs a new `Permission` instance with the given id, name, and key.
/// - `builder`: Returns a `PermissionBuilder` for constructing a `Permission` instance using the builder pattern.
pub struct Permission {
    id: i32,
    name: String,
    key: String,
}

impl Permission {
    pub fn new(id: i32, name: &str, key: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            key: String::from(key),
        }
    }

    pub fn builder() -> PermissionBuilder {
        PermissionBuilder::default()
    }
}
/// Builder for constructing a `Permission` instance.
///
/// Provides a fluent interface to incrementally set the fields of `Permission` and then build it.
///
/// Fields:
/// - `name`: Human-readable name of the permission (default-initialized).
/// - `key`: Unique key associated with the permission (default-initialized).
///
/// Methods:
/// - `name`: Sets the `name` field of the builder.
/// - `key`: Sets the `key` field of the builder.
/// - `build`: Constructs a `Permission` object with the specified fields. Note: `id` is default-initialized.
///
/// Example:
/// ```
/// let permission = PermissionBuilder::default()
///                      .name("Read Access")
///                      .key("read_access")
///                      .build();
/// ```
///
/// This example demonstrates creating a `Permission` object with specified `name` and `key` while leaving `id` default-initialized.
pub struct PermissionBuilder {
    id: i32,
    name: String,
    key: String,
}

impl Default for PermissionBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            key: Default::default(),
        }
    }
}

impl PermissionBuilder {
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    pub fn key(&mut self, key: &str) -> &mut Self {
        self.key = String::from(key);
        self
    }

    pub fn build(&self) -> Permission {
        Permission {
            id: Default::default(),
            name: Default::default(),
            key: Default::default(),
        }
    }
}
// ignore here todo;
pub struct PermissionRepo {
    pg: PostgresDatabase,
}

impl<'a> PermissionRepo {
    pub fn new(pg: PostgresDatabase) -> PermissionRepoBuilder<'a> {
        PermissionRepoBuilder { 
            pg, 
            action: PermissionAction::Create, 
            parameter: &[] 
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum PermissionAction {
    Create,
    Delete,
    CreateWithRole,
    DeleteWithRole
}

impl PermissionAction {
    fn to_query(&self) -> &'static str {
        match self {
            PermissionAction::Create => "INSERT INTO permission (permission_name, permission_key) VALUES ($1, $2)",
            PermissionAction::Delete => "DELETE FROM permission WHERE permission_key = $1",
            PermissionAction::CreateWithRole => "INSERT INTO role_permission (role_id, permission_id) VALUES ($1, $2)",
            PermissionAction::DeleteWithRole => "DELETE FROM role_permission WHERE role_id = $1 and permission_id = $2",
        }
    }

    fn parameter_amt(&self) -> usize {
        match self {
            PermissionAction::Create => 2,
            PermissionAction::Delete => 1,
            PermissionAction::CreateWithRole => 2,
            PermissionAction::DeleteWithRole => 2,
        }
    }
}

/// Builder for constructing and executing actions in a `PermissionRepo`.
///
/// Allows configuring a `PermissionRepo` operation with a specific database, action, and parameters.
///
/// Fields:
/// - `pg`: Instance of `PostgresDatabase` to interact with the database.
/// - `action`: The `PermissionAction` to be executed (e.g., Create, Read, Update, Delete).
/// - `parameter`: Slice of string slices representing additional parameters for the action.
///
/// Methods:
/// - `new`: Creates a new `PermissionRepoBuilder` with the given `PostgresDatabase` and default settings.
/// - `action`: Sets the action to be performed by the repository.
/// - `parameter`: Sets the parameters for the action.
/// - `execute`: Executes the configured action on the repository.
/// 
/// Parameters: 
/// - `Create`: ("permission name", "permission.key")
///
/// Example:
/// ```
/// let repo_builder = PermissionRepoBuilder::new(postgres_db)
///                      .action(PermissionAction::Create)
///                      .parameter(&["param1", "param2"])
///                      .execute();
/// ```
///
/// This example demonstrates setting up a `PermissionRepoBuilder` with a specific action and parameters, then executing the action.
pub struct PermissionRepoBuilder<'a> {
    pg: PostgresDatabase,
    action: PermissionAction,
    parameter: &'a [&'a str]
}

impl<'a> PermissionRepoBuilder<'a> {
    pub fn new(pg: PostgresDatabase) -> PermissionRepoBuilder<'a> {
        Self {
            pg,
            action: PermissionAction::Create,
            parameter: &[],
        }
    }

    pub fn action(&mut self, action: PermissionAction) -> &mut Self {
        self.action = action;
        self
    }

    pub fn parameter(&mut self, parameter: &'a [&str]) -> &mut Self {
        self.parameter = parameter;
        self
    }

    pub async fn execute(&self) {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool.prepare(self.action.to_query()).await.unwrap();
        if self.parameter.len() != self.action.parameter_amt() {
            // throw parameter mismatch
        }
        // pool.execute(&stmt, &[&""]).await.unwrap();
        todo!()
    }
}

pub fn test() {
    /*
    PermissionRepo::new(todo!())
        .action(PermissionAction::Create)
        .parameter(&[&"Create User", "user.create"])
        .execute();
    */
}

//static CREATE_PERMISSION_QUERY: &str = "INSERT INTO permission (permission_name, permission_key) VALUES ($1, $2)";

/*
static CREATE_PERMISSION_QUERY: &str = "INSERT INTO permission (permission_name, permission_key) VALUES ($1, $2)";
static DELETE_PERMISSION_QUERY: &str = "DELETE FROM permission WHERE permission_key = $1";
static CREATE_ROLE_PERMISSION_QUERY: &str = "INSERT INTO role_permission (role_id, permission_id) VALUES ($1, $2)";
static DELETE_ROLE_PERMISSION_QUERY: &str = "DELETE FROM role_permission WHERE role_id = $1 and permission_id = $2";
*/

//let x = "INSERT INTO permission (permission_name, permission_key) VALUES ($1, $2)";
//let stmt = pool.prepare(x).await.unwrap();

/*
pub struct Permission {
    id: usize,
    permission_name: String,
    permission_key: String,
}

impl Permission {
    pub fn new(id: usize, permission_name: &str, permission_key: &str) -> Self {
        Self {
            id,
            permission_name: String::from(permission_name),
            permission_key: String::from(permission_key),
        }
    }
}

#[derive(Clone)]
pub struct PermissionManager {
    pg: PostgresDatabase,
}

impl PermissionManager {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    pub async fn create_permission(
        &self,
        permission_name: &str,
        permission_key: &str,
    ) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO permission (permission_name, permission_key) VALUES ($1, $2)")
            .await?;
        let result = pool
            .execute(&stmt, &[&permission_name, &permission_key])
            .await?;
        Ok(result)
    }

    pub async fn delete_permission(&self, permission_key: &str) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("DELETE FROM permission WHERE permission_key = $1")
            .await?;
        let result = pool.execute(&stmt, &[&permission_key]).await?;
        Ok(result)
    }

    pub async fn add_role_permission(
        &self,
        role_id: i64,
        permission_id: i64,
    ) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO role_permission (role_id, permission_id) VALUES ($1, $2)")
            .await?;
        let result = pool.execute(&stmt, &[&role_id, &permission_id]).await?;
        Ok(result)
    }

    pub async fn delete_role_permission(
        &self,
        role_id: i32,
        permission_id: i32,
    ) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("DELETE FROM role_permission WHERE role_id = $1 and permission_id = $2")
            .await?;
        let result = pool.execute(&stmt, &[&role_id, &permission_id]).await?;
        Ok(result)
    }
}
*/
