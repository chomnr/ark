use crate::app::service::task::{
    manager::TaskManager,
    message::{TaskArgs, TaskRequest, TaskResponse, TaskType},
};

use super::{model::Permission, task::PermissionUpdateTask};

pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) -> TaskResponse {
        let request = TaskRequest::compose_request::<TaskArgs<Permission>>(
            TaskArgs::<Permission> { param: permission },
            TaskType::Permission,
            "permission_create",
        );
        TaskManager::send::<Permission>(request)
    }

    pub fn delete_permission(permission_identifer: &str) -> TaskResponse {
        let request = TaskRequest::compose_request::<TaskArgs<String>>(
            TaskArgs::<String> {
                param: String::from(permission_identifer),
            },
            TaskType::Permission,
            "permission_delete",
        );
        TaskManager::send::<String>(request)
    }

    pub fn update_permission(
        search_by: &str,
        update_for: &str,
        value: &str
    ) -> TaskResponse {
        let request = TaskRequest::compose_request::<TaskArgs<PermissionUpdateTask>>(
            TaskArgs::<PermissionUpdateTask> {
                param: PermissionUpdateTask {
                    search_by: String::from(search_by),
                    update_for: String::from(update_for),
                    value: String::from(value),
                },
            },
            TaskType::Permission,
            "permission_update",
        );
        TaskManager::send::<PermissionUpdateTask>(request)
    }
}
