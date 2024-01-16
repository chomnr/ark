use crate::app::service::task::{
    manager::TaskManager,
    message::{TaskRequest, TaskResponse, TaskType},
};

use super::{model::Permission, task::PermissionTask};

pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) -> TaskResponse {
        let request = TaskRequest::compose_request::<PermissionTask<Permission>>(
            PermissionTask::<Permission> { param: permission },
            TaskType::Permission,
            "permission_create",
        );
        TaskManager::send::<Permission>(request)
    }

    pub fn delete_permission(permission_identifer: &str) -> TaskResponse {
        let request = TaskRequest::compose_request::<PermissionTask<String>>(
            PermissionTask::<String> { param: String::from(permission_identifer) },
            TaskType::Permission,
            "permission_delete",
        );
        TaskManager::send::<String>(request)
    }
}
