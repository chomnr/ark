use tracing::{event, Level};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::iam::{account::error::UserRepositoryError, identity::model::UserIdentity},
};

use super::error::UserRepositoryResult;

static CREATE_IDENTITY_QUERY: &str =
    "INSERT INTO identity (username, email, oauth_provider, oauth_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (oauth_id)
            DO UPDATE SET last_login = CURRENT_TIMESTAMP
            RETURNING *;";

/// Repository for managing user-related data within a PostgreSQL database.
///
/// Fields:
/// - `pg`: An instance of `PostgresDatabase` representing the connection and operations specific to PostgreSQL.
pub(crate) struct UserRepository {
    pg: PostgresDatabase,
}

impl UserRepository {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    /// Asynchronously creates a new user identity in the repository.
    ///
    /// This function is responsible for adding a new `UserIdentity` record to the database.
    ///
    /// Arguments:
    /// - `identity`: Reference to a `UserIdentity` object containing the information for the new identity.
    ///
    /// Returns:
    /// - `UserRepositoryResult<()>`: A result type indicating success or failure of the operation.
    ///
    /// Note: Currently, the function body is not implemented (marked with `todo!()`).
    ///
    /// Example:
    /// ```
    /// let user_repo = UserRepository::new(database);
    /// let new_identity = UserIdentity {
    ///     id: 123,
    ///     username: "new_user".to_string(),
    ///     email: "user@example.com".to_string(),
    ///     // other fields...
    /// };
    /// let result = user_repo.create_new_identity(&new_identity).await;
    /// match result {
    ///     Ok(_) => println!("New identity created successfully"),
    ///     Err(e) => println!("Error creating new identity: {:?}", e),
    /// }
    /// ```
    pub async fn create_new_identity(&self, identity: &UserIdentity) -> UserRepositoryResult<()> {
        let client = self.pg.get().await;
        let stmt = client
            .prepare(CREATE_IDENTITY_QUERY)
            .await
            .expect("Error: failed to prepare query.");
        match client
            .execute(
                &stmt,
                &[
                    &identity.username,
                    &identity.email,
                    &identity.oauth_provider,
                    &identity.oauth_id,
                ],
            )
            .await
        {
            Ok(_) => {
                event!(
                    Level::INFO,
                    "[ARC] created a new identity for {}({})",
                    identity.username,
                    identity.id
                );
                Ok(())
            }
            Err(_) => {
                event!(
                    Level::ERROR,
                    "[ARC] failed to created an identity for {}({})",
                    identity.username,
                    identity.id
                );
                Err(UserRepositoryError::FailedToCreateIdentity)
            }
        }
    }
}
