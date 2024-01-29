use crate::app::service::task::{error::TaskResult, manager::TaskManager, message::{TaskRequest, TaskStatus, TaskType}};

use super::{model::User, task::UserCreateTask};


//use super::{task::UserCreateTask, model::User};

pub struct UserManager;

impl UserManager {
    pub fn create_user(user: User) -> TaskResult<TaskStatus> {
        let task_request = Self::create_user_request(user);
        TaskManager::process_task(task_request)
    }

    fn create_user_request(user: User) -> TaskRequest {
        TaskRequest::compose_request(
            UserCreateTask {
                user
            },
            TaskType::User,
            "user_create",
        )
    }
}