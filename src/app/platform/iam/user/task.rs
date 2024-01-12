use serde::{Deserialize, Serialize};

use crate::app::services::task::{model::{TaskMessage, TaskType}, manager::TaskManager};

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
            sql_1: String::from("INSERT INTO iam_users (id, username, email, created_at, updated_at)"),
            sql_2: String::from("INSERT INTO iam_user_oauth (user_id, oauth_id, oauth_provider)"),
            param: Default::default(),
        }
    }
}

/// Manages tasks related to user operations.
///
/// `UserTaskManager` is responsible for handling tasks such as creating, updating, or deleting 
/// user information. It provides functionality to orchestrate and execute various operations 
/// associated with user management.
pub struct UserTaskManager;

impl UserTaskManager {
    pub fn create_user(user: User) {
        let mut task = UserCreateTask::default();
        task.param = user;
        TaskManager::send(TaskMessage::compose(TaskType::User, "user_create_task", task));
    }
}


pub fn test(){
    let user = User::builder().build();
    UserTaskManager::create_user(user);
}