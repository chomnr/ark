use crate::app::services::task::{manager::TaskManager, model::{TaskMessage, TaskType}};

use super::{task::PermissionCreateTask, model::Permission};

pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) {
        let mut task = PermissionCreateTask::default();
        task.param = permission;
        TaskManager::send(TaskMessage::compose(TaskType::Permission, "permission_create", task));
    }
}