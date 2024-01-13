use axum::async_trait;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::app::database::postgres::PostgresDatabase;

use super::error::TaskResult;

#[derive(Clone, Copy)]
pub enum TaskType {
    Permission,
    Role,
    User,
}

impl TaskType {
    pub fn to_identifier(&self) -> String {
        match self {
            TaskType::Permission => return String::from("perm_task"),
            TaskType::Role => return String::from("role_task"),
            TaskType::User => return String::from("user_task"),
        }
    }
}

pub struct TaskMessage {
    pub task_id: String,
    pub task_action: String,
    pub task_type: TaskType,
    pub task_message: String,
}

impl TaskMessage {
    /// Creates a `TaskMessage` with a specific task type and a serialized message.
    ///
    /// # Arguments
    /// * `task_type` - The type of the task.
    /// * `task_action` - The action that will be executed.
    /// * `task_message` - The message to be serialized, of generic type `T`.
    ///
    /// # Returns
    /// Returns a `TaskMessage` with a unique task ID, task type, and serialized message.
    ///
    /// # Example
    /// ```
    /// let my_task = MyTask { /* ... */ };
    /// let task_message = TaskManager::compose(TaskType::MyTaskType, my_task);
    /// ```
    pub fn compose<T: for<'a> Deserialize<'a> + Serialize>(
        task_type: TaskType,
        task_action: &str,
        task_message: T,
    ) -> TaskMessage {
        TaskMessage {
            task_id: Self::generate_task_specific_id(task_type),
            task_action: String::from(task_action),
            task_type,
            task_message: serde_json::to_string(&task_message).unwrap(),
        }
    }

    /// Generates a unique identifier for a task based on its type.
    ///
    /// Combines the task type's identifier string with a unique ID.
    ///
    /// # Arguments
    /// * `task_type` - The type of the task.
    ///
    /// # Returns
    /// Returns a `String` representing the unique task-specific ID.
    ///
    /// # Example
    /// ```
    /// let task_id = generate_task_specific_id(TaskType::MyTaskType);
    /// ```
    fn generate_task_specific_id(sender_type: TaskType) -> String {
        format!("{}-{}", sender_type.to_identifier(), nanoid!(7))
    }
}
