use std::process::id;

use crate::app::service::task::{
    error::TaskResult,
    manager::TaskManager,
    message::{TaskRequest, TaskStatus, TaskType},
};

use super::{
    model::Role,
    task::{RoleCreateTask, RoleDeleteTask},
};

pub struct RoleManager;

impl RoleManager {
    pub fn create_role(role: Role) -> TaskResult<TaskStatus> {
        let task_request = Self::create_role_request(role);
        TaskManager::process_task(task_request)
    }

    /// Composes a role create request.
    ///
    /// # Arguments
    /// - `role`: A reference to the `Role` to process.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// Self::create_role_request(role)
    /// ```
    fn create_role_request(role: Role) -> TaskRequest {
        TaskRequest::compose_request(RoleCreateTask::from(role), TaskType::Role, "role_create")
    }
    
    // todo comment...
    pub fn delete_role(identifier: &str) -> TaskResult<TaskStatus> {
        let task_request = Self::delete_role_request(identifier);
        TaskManager::process_task(task_request)
    }

    /// Composes a role create request.
    ///
    /// # Arguments
    /// - `identifier`: You want to delete the role by it's id or name.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// Self::delete_role_request("fd817048-a5f0-47aa-94c3-f9b7b3a2265b")
    /// Self::delete_role_request("Administrator")
    /// ```
    fn delete_role_request(identifier: &str) -> TaskRequest {
        TaskRequest::compose_request(
            RoleDeleteTask {
                identifier: String::from(identifier),
            },
            TaskType::Role,
            "role_delete",
        )
    }
}
