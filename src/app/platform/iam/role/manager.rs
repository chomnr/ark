use std::process::id;

use crate::app::service::task::{
    error::TaskResult,
    manager::TaskManager,
    message::{TaskRequest, TaskStatus, TaskType},
};

use super::{
    model::Role,
    task::{RoleCreateTask, RoleDeleteTask, RolePreloadCache, RoleUpdateTask, RoleReadTask},
};

pub struct RoleManager;

impl RoleManager {
     /// Updates specific field within a role.
    ///
    /// # Arguments
    /// - `identifier`: Find a role based on it's identifier.
    ///
    /// # Examples
    /// ```
    /// let role = PermissionBuilder::builder()
    ///     .role_name("Member")
    ///     .build();
    /// create_role(role);
    /// ```
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

    /// Updates specific field within a role.
    ///
    /// # Arguments
    /// - `identifier`: Find a role based on it's identifier.
    ///
    /// # Examples
    /// ```
    /// delete_role("dd2546c3-e34a-4fcb-9b12-1a96eb6873e3");
    /// delete_role("Admin");
    /// ```
    pub fn delete_role(identifier: &str) -> TaskResult<TaskStatus> {
        let task_request = Self::delete_role_request(identifier);
        TaskManager::process_task(task_request)
    }

    /// Composes a role create request.
    ///
    /// # Arguments
    /// - `search_for`: Find a permission based on it's identifier.
    /// - `update_for`: The field that needs to be updated.
    /// - `value`: The value of the field.
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

    /// Updates specific field within a role.
    ///
    /// # Arguments
    /// - `search_for`: Find a role based on it's identifier.
    /// - `update_for`: The field that needs to be updated.
    /// - `value`: The value of the field.
    ///
    /// # Examples
    /// ```
    /// update_role("dd2546c3-e34a-4fcb-9b12-1a96eb6873e3", "role_name", "Admin");
    /// update_role("Admin", "role_name", "Administrator");
    /// ```
    pub fn update_role(search_by: &str, update_for: &str, value: &str) -> TaskResult<TaskStatus> {
        let request = Self::update_role_request(search_by, update_for, value);
        TaskManager::process_task(request)
    }

    /// Composes a role update request.
    ///
    /// # Arguments
    /// - `search_for`: Find a role based on it's identifier.
    /// - `update_for`: The field that needs to be updated.
    /// - `value`: The value of the field.
    ///
    /// # Examples
    /// ```
    /// let task_response = update_role_request("Administrator", "role_name", "Admin");
    /// ```
    fn update_role_request(search_by: &str, update_for: &str, value: &str) -> TaskRequest {
        TaskRequest::compose_request::<RoleUpdateTask>(
            RoleUpdateTask {
                search_by: search_by.to_string(),
                update_for: update_for.to_string(),
                value: value.to_string(),
            },
            TaskType::Role,
            "role_update",
        )
    }

    /// Read a specific role
    ///
    /// # Arguments
    /// - `identifier`: Find a role based on it's identifier.
    ///
    /// # Examples
    /// ```
    /// get_role("Administrator");
    /// ```
    pub fn get_role(identifier: &str) -> TaskResult<Role> {
        let request = Self::read_role_request(identifier);
        TaskManager::process_task_with_result::<Role>(request)
    }

    /// Composes a role update request.
    ///
    /// # Arguments
    /// - `identifier`: Find a role based on it's identifier.
    ///
    /// # Examples
    /// ```
    /// let task_response = read_role_request("Administrator");
    /// ```
    fn read_role_request(identifier: &str) -> TaskRequest {
        TaskRequest::compose_request::<RoleReadTask>(
            RoleReadTask {
                identifier: String::from(identifier),
            },
            TaskType::Role,
            "role_read",
        )
    }

    /// Preload role cache.
    ///
    /// # Examples
    /// ```
    /// // Assuming `role` is a reference to a valid Permission
    /// preload_role_cache();
    /// ```
    pub fn preload_role_cache() -> TaskResult<TaskStatus> {
        let request = Self::preload_role_request();
        TaskManager::process_task(request)
    }

    /// Composes a permission preload cache request.
    ///
    /// # Examples
    /// ```
    /// let task_response = preload_permission_request();
    /// ```
    fn preload_role_request() -> TaskRequest {
        TaskRequest::compose_request(RolePreloadCache {}, TaskType::Role, "role_preload_cache")
    }
}
