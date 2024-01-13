use thiserror::Error;

pub type TaskResult<T> = Result<T, TaskError>;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("TaskUniqueConstraint")]
    TaskUniqueConstraint,
    #[error("TaskInvalid")]
    TaskInvalid
}