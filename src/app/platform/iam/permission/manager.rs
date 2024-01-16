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
        let response = TaskManager::send::<Permission>(request);
        println!("{:?} test ", response.task_status);
        response
    }
}

/*
pub struct PermissionManager;

impl PermissionManager {
    pub fn create_permission(permission: Permission) {
        match TaskManager::send::<bool>(TaskMessage::compose(
            "permission",
            "permission_create",
            permission,
        )) {
            Ok(_) => println!("successful"),
            Err(_) => println!("failed"),
        }
    }
}
*/
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
