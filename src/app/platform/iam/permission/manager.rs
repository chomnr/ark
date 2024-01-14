use crate::app::services::task::{
    error::TaskResult,
    manager::TaskManager,
    model::{TaskMessage, TaskType},
};

use super::model::Permission;

pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) {
        let task = TaskManager::send::<Permission>(TaskMessage::compose(
            TaskType::Permission,
            "permission_create",
            permission,
        ));
    }
}

//pub fn test() {
    //PermissionManager::create_permission(todo!())
    //    let permission = Permission::builder()
    //    .permission_name("permission_name")
    //    .permission_key("permission_key")
    //    .build();
    //     PermissionManager::create_permission(permission);
//}

// PermissionManager::delete_permission("sdd".to_string(), "a".to_string())

//PermissionManager::delete_permission("dsadasads")
//    .listen();

/*
use super::{task::PermissionCreateTask, model::Permission};

pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) {
        let mut task = PermissionCreateTask::default();
        task.param = permission;
        TaskManager::send(TaskMessage::compose(TaskType::Permission, "permission_create", task));
    }
}
*/
