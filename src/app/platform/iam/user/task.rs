use serde::{Deserialize, Serialize};
use syn::token::Use;


use crate::app::services::task::{model::{TaskType, TaskMessage}, manager::TaskManager};

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