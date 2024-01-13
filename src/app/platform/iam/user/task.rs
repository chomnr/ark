use serde::{Deserialize, Serialize};

use crate::app::database::postgres::PostgresDatabase;

use super::model::User;

/// Represents a task for creating a new user, containing SQL statements and user parameters.
#[derive(Serialize, Deserialize)]
pub struct UserCreateTask {
    pub sql_1: String,
    pub sql_2: String,
    pub sql_3: String,
    pub sql_4: String,
    pub param: User,
}

impl Default for UserCreateTask {
    fn default() -> Self {
        Self {
            sql_1: String::from("INSERT INTO iam_users (id, username, email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"),
            sql_2: String::from("INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider) VALUES ($1, $2, $3)"),
            sql_3: String::from("INSERT INTO iam_roles (id, role_name) VALUES ($1, $2)"),
            sql_4: String::from("INSERT INTO iam_permissions (id, permission_name, permission_key) VALUES($1, $2, $3)"),
            param: Default::default(),
        }
    }
}

impl UserCreateTask {
    /// Creates a new instance of `UserCreateTask` with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// let user_create_task = UserCreateTask::new();
    /// // `user_create_task` is now ready with default SQL queries and parameters.
    /// ```
    pub fn new() -> Self {
        UserCreateTask::default()
    }

    /// Asynchronously processes a user creation task using a PostgreSQL database connection.
    ///
    /// # Arguments
    ///
    /// * `pg` - A reference to `PostgresDatabase`, providing the necessary database connection.
    ///
    /// # Examples
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let pg_database = PostgresDatabase::new(/* database connection parameters */);
    ///     let user_create_task = UserCreateTask::new();
    ///     user_create_task.process(&pg_database).await;
    /// }
    /// ```
    pub async fn process(&self, pg: &PostgresDatabase) {
        let mut pool = pg.pool.get().await.unwrap();
        // start transaction
        let mut transaction = pool.transaction().await.unwrap();
        
        transaction.commit().await.unwrap();
        // check if user is in cache
        // check if user exists.
        // then process transaction.
    }
}