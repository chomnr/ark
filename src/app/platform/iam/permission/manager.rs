use crate::app::service::task::{
    error::{TaskError, TaskResult},
    manager::TaskManager,
    message::{TaskRequest, TaskStatus, TaskType},
};

use super::{
    model::Permission,
    task::{PermissionCreateTask, PermissionDeleteTask, PermissionUpdateTask},
};

pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) -> TaskResult<TaskStatus> {
        let request = TaskRequest::compose_request::<PermissionCreateTask>(
            PermissionCreateTask {
                permission_id: permission.permission_id,
                permission_name: permission.permission_name,
                permission_key: permission.permission_key,
            },
            TaskType::Permission,
            "permission_create",
        );
        let task = TaskManager::send(request);
        match task.task_status {
            TaskStatus::Completed => {
                return Ok(task.task_status);
            }
            TaskStatus::Failed => return Err(TaskError::FailedToCompleteTask),
        }
    }

    pub fn delete_permission(permission_identifer: &str) -> TaskResult<TaskStatus> {
        let request = TaskRequest::compose_request::<PermissionDeleteTask>(
            PermissionDeleteTask {
                identifier: String::from(permission_identifer),
            },
            TaskType::Permission,
            "permission_delete",
        );
        let task = TaskManager::send(request);
        match task.task_status {
            TaskStatus::Completed => {
                return Ok(task.task_status);
            }
            TaskStatus::Failed => return Err(TaskError::FailedToCompleteTask),
        }
    }

    pub fn update_permission(
        search_by: &str,
        update_for: &str,
        value: &str,
    ) -> TaskResult<TaskStatus> {
        let request = TaskRequest::compose_request::<PermissionUpdateTask>(
            PermissionUpdateTask {
                search_by: String::from(search_by),
                update_for: String::from(update_for),
                value: String::from(value),
            },
            TaskType::Permission,
            "permission_update",
        );
        let task = TaskManager::send(request);
        match task.task_status {
            TaskStatus::Completed => {
                return Ok(task.task_status);
            }
            TaskStatus::Failed => return Err(TaskError::FailedToCompleteTask),
        }
    }

    pub fn get_permission(permission_identifier: &str) -> Permission {
        todo!()
    }
}
