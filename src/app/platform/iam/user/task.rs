use axum::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{
    database::postgres::PostgresDatabase,
    services::task::{error::{TaskResult, TaskError}, model::TaskAction},
};

use super::model::User;

/// Represents a task for creating a new user, containing SQL statements and user parameters.
///
/// This struct holds two SQL statements for inserting data into `iam_users` and `iam_user_oauth`
/// tables, along with a `User` object containing the user data to be inserted.
#[derive(Serialize, Deserialize)]
pub struct UserCreateTask {
    pub sql_1: String,
    pub sql_2: String,
    pub param: User,
}

impl Default for UserCreateTask {
    fn default() -> Self {
        Self {
            sql_1: String::from("INSERT INTO iam_users (id, username, email, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"),
            sql_2: String::from("INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider) VALUES ($1, $2, $3)"),
            param: Default::default(),
        }
    }
}

/*
impl UserCreateTask {
    fn from_task_message(task_message: String) -> Self {
        let task: UserCreateTask = serde_json::from_str(&task_message).unwrap();
        task
    }

    async fn create_user(
        pg: &PostgresDatabase,
        user_create_task: UserCreateTask,
    ) -> TaskResult<()> {
        let mut pool = pg.pool.get().await.unwrap();
        let transaction = pool.transaction().await.unwrap();
        Ok(())
    }
}

#[async_trait]
impl TaskAction for UserCreateTask {
    async fn process(
        pg: &PostgresDatabase,
        task_action: String,
        task_message: String,
    ) -> TaskResult<()> {
        let task = Self::from_task_message(task_message);
        if task_action.eq("user_create") {
            Self::create_user(pg, task).await?;
            return Ok(())
        }
        Err(TaskError::TaskInvalid)
    }
}
*/