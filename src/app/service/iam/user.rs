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
/// The structure is designed to capture essential user-related data, including identification, contact information, and acc
pub struct User {
    id: i64,
    username: String,
    email: String,
    verified: bool,
    created_at: i64,
    updated_at: i64,
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
