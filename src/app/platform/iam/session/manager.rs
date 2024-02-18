use uuid::Uuid;

use crate::app::service::task::{
    error::TaskResult,
    manager::TaskManager,
    message::{TaskRequest, TaskType},
};

use super::{model::UserSession, task::SessionCreateTask};

pub struct SessionManager;

impl SessionManager {
    /// Create a user session.
    ///
    /// # Arguments
    /// - `user_id`: who to create the sessionf or.
    ///
    /// # Examples
    /// ```
    /// let role = PermissionBuilder::builder()
    ///     .role_name("Member")
    ///     .build();
    /// create_role(role);
    /// ```
    pub fn create_session(user_id: &str) -> TaskResult<UserSession> {
        let task_request = Self::create_session_request(UserSession {
            token: Uuid::new_v4().as_simple().to_string(),
            expires_in: 604800,
            user_id: user_id.to_string(),
        });
        TaskManager::process_task_with_result::<UserSession>(task_request)
    }

    /// Composes a user session create request.
    ///
    /// # Arguments
    /// - `session`: A reference to the `Session` to process.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// Self::create_role_request(role)
    /// ```
    fn create_session_request(session: UserSession) -> TaskRequest {
        TaskRequest::compose_request(
            SessionCreateTask {
                token: session.token,
                expires_in: session.expires_in,
                user_id: session.user_id,
            },
            TaskType::Session,
            "session_create",
        )
    }
}
