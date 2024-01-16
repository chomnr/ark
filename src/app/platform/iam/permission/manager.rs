use crate::app::service::task::{
    message::{TaskRequest, TaskType, TaskResponse}, manager::TaskManager,
};

use super::{model::Permission, task::PermissionTask};

pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) -> TaskResponse {
        let request = TaskRequest::compose_request::<PermissionTask<Permission>>(
            PermissionTask::<Permission> {
                action: "permission_create".to_string(),
                param: permission,
            },
            TaskType::Permission,
        );
        TaskManager::send::<Permission>(request)
    }
}