use bb8_postgres::tokio_postgres::types::ToSql;

use crate::app::database::postgres::PostgresDatabase;

use super::error::{IamError, IamResult};

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
/// 
/// Example:
/// ```
/// let permission = Role::new(1, "Ban User", "ban.user");
/// ```
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
    pub fn id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

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
            id: self.id,
            name: self.name.clone(),
            key: self.key.clone(),
        }
    }
}
/// Repository for managing permission-related data in a database.
///
/// Encapsulates database interactions specifically for permission-related operations.
///
/// Field:
/// - `pg`: An instance of `PostgresDatabase` representing the database connection and operations.
///
/// Methods:
/// - `new`: Initializes a `PermissionRepoBuilder` with default settings for creating permissions.
///
/// Example:
/// ```
/// let permission_repo = PermissionRepo::new(postgres_database);
/// // Use permission_repo to perform permission-related operations.
/// ```
///
/// The `new` method creates a `PermissionRepoBuilder` preset to create new permissions, allowing further customization through the builder's methods.
pub struct PermissionRepo;

#[derive(PartialEq, Eq)]
pub enum PermissionAction {
    Create,
    Delete,
    CreateWithRole,
    DeleteWithRole,
}

impl<'a> PermissionRepo {
    pub fn new(pg: PostgresDatabase) -> PermissionRepoBuilder<'a> {
        PermissionRepoBuilder {
            pg,
            action: PermissionAction::Create,
            parameter: &[],
        }
    }
}

impl PermissionAction {
    fn to_query(&self) -> &'static str {
        match self {
            PermissionAction::Create => {
                "INSERT INTO permission (permission_name, permission_key) VALUES ($1, $2)"
            }
            PermissionAction::Delete => "DELETE FROM permission WHERE permission_key = $1",
            PermissionAction::CreateWithRole => {
                "INSERT INTO role_permission (role_id, permission_id) VALUES ($1, $2)"
            }
            PermissionAction::DeleteWithRole => {
                "DELETE FROM role_permission WHERE role_id = $1 and permission_id = $2"
            }
        }
    }

    fn error(&self) -> IamError {
        match self {
            PermissionAction::Create => IamError::PermissionCreationFailed,
            PermissionAction::Delete => IamError::PermissionDeletionFailed,
            PermissionAction::CreateWithRole => IamError::PermissionCreationFailed,
            PermissionAction::DeleteWithRole => IamError::PermissionFailedToDeleteLinkToRole,
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
/// - `Create`: ("permission_name", "permission_key")
/// - `Delete`: ("permission_key")
/// - `CreateWithRole`: ("role_id", "permission_id")
/// - `DeleteWithRole`: ("role_id", "permission_id")
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
    parameter: &'a [&'a (dyn ToSql + Sync)],
}

impl<'a> PermissionRepoBuilder<'a> {
    pub fn action(&mut self, action: PermissionAction) -> &mut Self {
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