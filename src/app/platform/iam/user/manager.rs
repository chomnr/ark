use syn::token::Use;

use crate::app::services::task::{model::{TaskMessage, TaskType}, manager::TaskManager};

use super::{task::UserCreateTask, model::User};

pub struct UserManager;

impl UserManager {
    pub fn create_user(user: User) {
        let mut task = UserCreateTask::default();
        task.param = user;
        TaskManager::send(TaskMessage::compose(TaskType::User, "user_create", task));
    }
}

/* 
pub struct UserManager;

impl UserManager {
    pub fn create_user(user: User) {
        let mut task = UserCreateTask::default();
        task.param = user;
        //TaskManager::send(TaskMessage::compose(TaskType::User, "user_create", task));
    }
}
*/