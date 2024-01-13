use serde::{Deserialize, Serialize};

use crate::app::{database::postgres::PostgresDatabase, services::task::error::{TaskResult, TaskError}};

use super::model::User;

/// Represents a task for creating a new user, containing SQL statements and user parameters.
#[derive(Serialize, Deserialize)]
pub struct UserCreateTask {
    sql_1: String,
    sql_2: String,
    sql_3: String,
    sql_4: String,
    pub param: User,
}

impl Default for UserCreateTask {
    fn default() -> Self {
        Self {
            sql_1: String::from("INSERT INTO iam_users (id, username, email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"),
            sql_2: String::from("INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider) VALUES ($1, $2, $3)"),
            sql_3: String::from("INSERT INTO iam_roles (id, role_name) VALUES ($1, $2)"),
            sql_4: String::from("todo"),
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
    pub async fn process(&self, pg: &PostgresDatabase) -> TaskResult<()> {
        let mut pool = pg.pool.get().await.unwrap();
        let mut transaction = pool.transaction().await.unwrap();
        match transaction.commit().await {
            Ok(_) => Ok(()),
            Err(_) => Err(TaskError::TaskWentWrong),
        }
        // check if user is in cache
        // check if user exists.
        // then process transaction.
    }
}