use crate::app::service::task::{
    manager::TaskManager,
    message::{TaskRequest, TaskResponse, TaskType, TaskArgs},
};

use super::model::Permission;

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
            TaskArgs::<String> { param: String::from(permission_identifer) },
            TaskType::Permission,
            "permission_delete",
        );
        TaskManager::send::<String>(request)
    }
}