use bb8_postgres::tokio_postgres::types::ToSql;
use serde::{Deserialize, Serialize};

use crate::app::{database::postgres::PostgresDatabase, service::iam::error::{IamError, IamResult}};

/// Represents a user in the system.
///
/// This struct encapsulates key information and attributes associated with a user.
///
/// Fields:
/// - `id`: An `i32` representing the unique identifier of the user.
/// - `username`: A `String` containing the user's username.
/// - `email`: A `String` representing the user's email address.
/// - `verified`: A `bool` indicating whether the user's account has been verified.
/// - `created_at`: A `String` specifying the timestamp when the user account was created.
/// - `updated_at`: A `String` indicating the timestamp of the last update made to the user account.
///
/// The structure is designed to capture essential user-related data, including identification, contact information, and account
#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub verified: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    pub fn new(
        id: i64,
        username: &str,
        email: &str,
        verified: bool,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            id,
            username: String::from(username),
            email: String::from(email),
            verified,
            created_at,
            updated_at,
        }
    }

    pub fn builder() -> UserBuilder {
        UserBuilder::default()
    }
}
/// Builder for constructing a `User` instance.
///
/// Provides a flexible way to build a `User` object, allowing for incremental setting of user attributes.
///
/// Fields:
/// - `id`: A `i64` representing the unique identifier of the user.
/// - `username`: A `String` for the user's username.
/// - `email`: A `String` for the user's email address.
/// - `verified`: A `bool` indicating whether the user's account is verified.
/// - `created_at`: A `String` denoting the timestamp when the user account was created.
/// - `updated_at`: A `String` representing the timestamp of the last update to the user account.
///
/// The `UserBuilder` pattern allows for more readable and maintainable code when creating `User` objects with multiple fields.
///
/// Example:
/// ```
/// let user = UserBuilder {
///     id: 1,
///     username: "user123".to_string(),
///     email: "user@example.com".to_string(),
///     verified: true,
///     created_at: 000000,
///     updated_at: 000000,
/// }.build();
/// ```
///
/// This example demonstrates creating a `User` object with specified details using the `UserBuilder`.
pub struct UserBuilder {
    id: i64,
    username: String,
    email: String,
    verified: bool,
    created_at: i64,
    updated_at: i64,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: Default::default(),
            username: Default::default(),
            email: Default::default(),
            verified: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

impl UserBuilder {
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.id = id;
        self
    }

    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = String::from(username);
        self
    }

    pub fn email(&mut self, email: &str) -> &mut Self {
        self.email = String::from(email);
        self
    }

    pub fn verified(&mut self, verified: bool) -> &mut Self {
        self.verified = verified;
        self
    }

    pub fn created_at(&mut self, created_at: i64) -> &mut Self {
        self.created_at = created_at;
        self
    }

    pub fn updated_at(&mut self, updated_at: i64) -> &mut Self {
        self.updated_at = updated_at;
        self
    }

    pub fn build(&self) -> User {
        User {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone(),
            verified: self.verified,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
/// Repository for managing user-related data within the system.
///
/// `UserRepo` is designed to handle various database operations related to users, such as creating, updating, or disabling user accounts.
/// It typically interacts with a database or other persistent storage mechanisms to perform these operations.
///
/// The implementation of `UserRepo` should define methods that carry out specific user-related tasks using database queries or transactions.
///
/// Note: The current implementation of `UserRepo` is empty (`impl<'a> UserRepo {}`). It should be expanded with methods for user data management.
pub struct UserRepo;

#[derive(PartialEq, Eq)]
pub enum UserAction {
    Create,
}

impl UserAction {
    fn to_query(&self) -> &'static str {
        match self {
            UserAction::Create => "INSERT INTO users (verified) VALUES ($1)",
        }
    }

    fn error(&self) -> IamError {
        match self {
            UserAction::Create => IamError::UserCreationFailed,
        }
    }

    fn parameter_amt(&self) -> usize {
        match self {
            UserAction::Create => 1,
        }
    }
}
/// Builder for creating and configuring a `UserRepo` instance for user-related database operations.
///
/// This struct provides a way to set up a `UserRepo` with specific database settings and operational parameters.
/// It uses the builder pattern for incremental and flexible configuration.
///
/// Fields:
/// - `pg`: An instance of `PostgresDatabase`, representing the database connection and operations.
/// - `action`: A `UserAction` enum specifying the type of action (e.g., Create, Disable) to perform on user data.
/// - `parameter`: A slice of references to objects that implement `ToSql` and `Sync`, representing the SQL parameters for the action.
///
/// Lifetimes:
/// - `'a`: Ensures that the references in the `parameter` field are valid for the duration of the `UserRepoBuilder`.
///
/// The builder allows for specifying the action to be taken on the user repository and the parameters required for that action, facilitating complex user data management tasks.
///
/// Example:
/// ```
/// let user_repo_builder = UserRepoBuilder {
///     pg: postgres_database,
///     action: UserAction::Create,
///     parameter: &[&"username", &"email"],
/// };
/// // Additional configuration can be applied to the builder here.
/// // Finally, build and execute the action on the UserRepo.
/// ```
pub struct UserRepoBuilder<'a> {
    pg: PostgresDatabase,
    action: UserAction,
    parameter: &'a [&'a (dyn ToSql + Sync)],
}

impl<'a> UserRepoBuilder<'a> {
    pub fn action(&mut self, action: UserAction) -> &mut Self {
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