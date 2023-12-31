use bb8_postgres::tokio_postgres::types::ToSql;

use crate::app::database::postgres::PostgresDatabase;

use super::error::{IamError, IamResult};

/// Represents a user role within the system.
///
/// Fields:
/// - `id`: An `i32` identifier for the role.
/// - `role_name`: A `String` representing the name of the role.
///
/// Methods:
/// - `new`: Constructor for creating a new `Role`. It takes an `id` and a `role_name` and returns an instance of `Role`.
///
/// Example:
/// ```
/// let admin_role = Role::new(1, "Administrator");
/// ```
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
        RoleBuilder::default()
    }
}

/// Builder for creating instances of `Role`.
///
/// This builder pattern allows for constructing a `Role` object with optional configurations.
/// Each field can be set independently before finalizing the creation of a `Role` instance.
///
/// Fields:
/// - `id`: An `i32` representing the identifier for the role.
/// - `role_name`: A `String` specifying the name of the role.
///
/// Example:
/// ```
/// let role_builder = RoleBuilder {
///     id: 2,
///     role_name: "Manager".to_string(),
/// };
/// let manager_role = role_builder.build();
/// ```
///
/// This example demonstrates creating a `Role` for a manager using the `RoleBuilder`.
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
            id: Default::default(),
            name: Default::default(),
        }
    }
}

/// Repository for managing role data within a database.
///
/// `RoleRepo` is responsible for handling storage, retrieval, update, and deletion operations related to roles.
/// It typically interacts with a database or other storage mechanisms to perform these operations.
///
/// This struct does not contain fields but serves as an encapsulation for role-related data access functionalities.
///
/// Example Usage:
/// ```
/// let role_repo = RoleRepo::new();
/// // Use role_repo to perform operations like adding, retrieving, or updating roles.
/// ```
///
/// In this example, `role_repo` is an instance of `RoleRepo` used for role data management operations.
pub struct RoleRepo;

#[derive(PartialEq, Eq)]
pub enum RoleAction {
    Create,
    Delete
}

impl<'a> RoleRepo {
    pub fn new(pg: PostgresDatabase) -> RoleRepoBuilder<'a> {
        RoleRepoBuilder {
            pg,
            action: RoleAction::Create,
            parameter: &[],
        }
    }
}

impl RoleAction {
    fn to_query(&self) -> &'static str {
        match self {
            RoleAction::Create => "INSERT INTO role (role_name) VALUES ($1)",
            RoleAction::Delete => "DELETE FROM role WHERE role_name = $1",
        }
    }

    fn error(&self) -> IamError {
        match self {
            RoleAction::Create => IamError::RoleCreationFailed,
            RoleAction::Delete => IamError::RoleDeletionFailed,
        }
    }

    fn parameter_amt(&self) -> usize {
        match self {
            RoleAction::Create => 1,
            RoleAction::Delete => 1,
        }
    }
}

/// Builder for constructing and executing actions within a `RoleRepo`.
///
/// This builder facilitates setting up and performing database operations related to role management.
///
/// Fields:
/// - `pg`: Instance of `PostgresDatabase` for interacting with the database.
/// - `action`: The type of `RoleAction` to perform (e.g., Create, Delete).
/// - `parameter`: A slice of references to objects implementing `ToSql` and `Sync`, representing SQL parameters.
///
/// Methods:
/// - `action`: Sets the action to be performed in the repository.
/// - `parameter`: Sets the parameters for the action.
/// - `execute`: Asynchronously executes the configured action in the database.
/// 
/// Parameters:
/// - `Create`: ("role_name")
/// - `Delete`: ("role_name")
///
/// Example:
/// ```
/// let builder = RoleRepoBuilder {
///     pg: postgres_database_instance,
///     action: RoleAction::Create,
///     parameter: &[&"role_name"],
/// };
/// let result = builder.action(RoleAction::Create)
///                      .parameter(&[&"role_name"])
///                      .execute()
///                      .await;
/// ```
///
/// In this example, `builder` is configured to create a new role with the specified parameters, and then executes this action asynchronously.
pub struct RoleRepoBuilder<'a> {
    pg: PostgresDatabase,
    action: RoleAction,
    parameter: &'a [&'a (dyn ToSql + Sync)],
}

impl<'a> RoleRepoBuilder<'a> {
    pub fn action(&mut self, action: RoleAction) -> &mut Self {
        self.action = action;
        self
    }

    pub fn parameter(&mut self, parameter: &'a [&'a (dyn ToSql + Sync)]) -> &mut Self {
        self.parameter = parameter;
        self
    }

    pub async fn execute(&self) -> IamResult<u64> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool.prepare(self.action.to_query()).await.unwrap();
        if self.parameter.len() != self.action.parameter_amt() {
            return Err(IamError::ParameterMismatch);
        }
        match pool.execute(&stmt, self.parameter).await {
            Ok(v) => Ok(v),
            Err(_) => Err(self.action.error()),
        }
    }
}