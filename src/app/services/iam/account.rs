mod account_queries {
    pub static CREATE_USER: &str = r#"INSERT INTO users (username, email, oauth_provider, oauth_id)
                                    VALUES ($1, $2, $3, $4)
                                    ON CONFLICT (oauth_id) 
                                    DO UPDATE SET last_login = CURRENT_TIMESTAMP
                                    RETURNING *;"#;
}

/// Represents a user account in the system.
///
/// Fields:
/// - `id`: Unique identifier for the account.
/// - `username`: The user's chosen username.
/// - `email`: Email address associated with the account.
/// - `verified`: Indicates whether the account has been verified.
/// - `oauth_provider`: The OAuth provider used for authentication (if applicable).
/// - `oauth_id`: The OAuth identifier from the authentication provider.
/// - `created_at`: Timestamp of account creation.
/// - `last_login`: Timestamp of the user's last login.
pub struct Account {
    id: usize,
    username: String,
    email: String,
    verified: bool,
    oauth_provider: String,
    oauth_id: String,
    created_at: String,
    last_login: String,
}

impl Account {
    /// Creates a new `Account` instance with the specified details.
    ///
    /// This constructor initializes an account with provided username, email, OAuth provider, and OAuth ID.
    /// The `id` is set to 0, `verified` status is set to false, and timestamps for creation and last login are set to default.
    ///
    /// Arguments:
    /// - `username`: The username for the new account.
    /// - `email`: The email address associated with the account.
    /// - `oauth_provider`: The OAuth provider used for authentication.
    /// - `oauth_id`: The identifier from the OAuth provider.
    ///
    /// Returns a new `Account` instance.
    ///
    /// Example:
    /// ```
    /// let account = Account::new("username", "email@example.com", "OAuthProvider", "OAuthID");
    /// ```
    pub fn new(username: &str, email: &str, oauth_provider: &str, oauth_id: &str) -> Self {
        Self {
            id: 0,
            username: String::with_capacity(username.len()),
            email: String::with_capacity(email.len()),
            verified: false,
            oauth_provider: String::with_capacity(oauth_provider.len()),
            oauth_id: String::with_capacity(oauth_id.len()),
            created_at: String::default(),
            last_login: String::default(),
        }
    }
}

// https://news.ycombinator.com/item?id=38720544
// when using oauth2 ensure that the users verifies
// the email on their own.
