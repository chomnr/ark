use crate::app::service::task::{
    error::TaskResult,
    manager::TaskManager,
    message::{TaskRequest, TaskStatus, TaskType},
};

use super::{
    model::Permission,
    task::{
        PermissionCreateTask, PermissionDeleteTask, PermissionPreloadCache, PermissionUpdateTask,
    },
};

pub struct PermissionManager;

impl PermissionManager {
    /// Create a permission.
    ///
    /// # Arguments
    /// - `permission`: A reference to the `Permission` for your desired permission.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// let permission = PermissionBuilder::builder()
    ///     .permission_name("admin ban user")
    ///     .permission_key("admin.ban")
    ///     .build();
    /// let task_response = create_permission(permission);
    /// ```
    pub fn create_permission(permission: Permission) -> TaskResult<TaskStatus> {
        let task_request = Self::create_permission_request(permission);
        TaskManager::process_task(task_request)
    }

    /// Composes a permission create request.
    ///
    /// # Arguments
    /// - `permission`: A reference to the `Permission` to process.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// Self::process_permission_task(permission)
    /// ```
    fn create_permission_request(permission: Permission) -> TaskRequest {
        TaskRequest::compose_request(
            PermissionCreateTask::from(permission),
            TaskType::Permission,
            "permission_create",
        )
    }

    /// Delete a permission.
    ///
    /// # Arguments
    /// - `permission_identifer`: Deletes a permission based on it's identifier ex: id, name, or key.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// let permission = PermissionBuilder::builder()
    ///     .permission_name("admin ban user")
    ///     .permission_key("admin.ban")
    ///     .build();
    /// delete_permission("dd2546c3-e34a-4fcb-9b12-1a96eb6873e3"); // delete by id.
    /// delete_permission("Testing name") // delete by name
    /// delete_permission("Testing.key") // delete by key
    /// ```
    pub fn delete_permission(permission_identifer: &str) -> TaskResult<TaskStatus> {
        let task_request = Self::delete_permission_request(permission_identifer);
        TaskManager::process_task(task_request)
    }

    /// Composes a permission delete request.
    ///
    /// # Arguments
    /// - `identifier`: Deletes a permission based on it's identifier ex: id, name, or key.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// delete_permission_request("dd2546c3-e34a-4fcb-9b12-1a96eb6873e3");
    /// ```
    fn delete_permission_request(identifier: &str) -> TaskRequest {
        TaskRequest::compose_request::<PermissionDeleteTask>(
            PermissionDeleteTask {
                identifier: identifier.to_string(),
            },
            TaskType::Permission,
            "permission_delete",
        )
    }

    /// Updates specific field within a permission.
    ///
    /// # Arguments
    /// - `search_for`: Find a permission based on it's identifier.
    /// - `update_for`: The field that needs to be updated.
    /// - `value`: The value of the field.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// update_permission("dd2546c3-e34a-4fcb-9b12-1a96eb6873e3", "permission_name", "admin ban user.");
    /// update_permission("admin ban user.", "permission_key", "admin.ban.key");
    /// ```
    pub fn update_permission(
        search_by: &str,
        update_for: &str,
        value: &str,
    ) -> TaskResult<TaskStatus> {
        let request = Self::update_permission_request(search_by, update_for, value);
        TaskManager::process_task(request)
    }

    /// Composes a permission update request.
    ///
    /// # Arguments
    /// - `search_for`: Find a permission based on it's identifier.
    /// - `update_for`: The field that needs to be updated.
    /// - `value`: The value of the field.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// let permission = PermissionBuilder::builder()
    ///     .permission_name("admin ban user")
    ///     .permission_key("admin.ban")
    ///     .build();
    /// let task_response = create_permission(permission);
    /// ```
    fn update_permission_request(search_by: &str, update_for: &str, value: &str) -> TaskRequest {
        TaskRequest::compose_request::<PermissionUpdateTask>(
            PermissionUpdateTask {
                search_by: search_by.to_string(),
                update_for: update_for.to_string(),
                value: value.to_string(),
            },
            TaskType::Permission,
            "permission_update",
        )
    }

    pub fn get_permission(permission_identifier: &str) -> Permission {
        todo!()
    }

    /// Preload permission cache.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// preload_permission_cache();
    /// ```
    pub fn preload_permission_cache() -> TaskResult<TaskStatus> {
        let request = Self::preload_permission_request();
        TaskManager::process_task(request)
    }

    /// Composes a permission preload cache request.
    ///
    /// # Examples
    /// ```
    /// let task_response = preload_permission_request();
    /// ```
    fn preload_permission_request() -> TaskRequest {
        TaskRequest::compose_request(
            PermissionPreloadCache {},
            TaskType::Permission,
            "permission_preload_cache",
        )
    }
}
