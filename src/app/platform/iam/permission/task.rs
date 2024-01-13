use serde::{Deserialize, Serialize};

use crate::app::{database::postgres::PostgresDatabase, services::task::error::{TaskResult, TaskError}};

use super::model::Permission;

/// Represents a task for creating a new permission, containing SQL statements and user parameters.
#[derive(Serialize, Deserialize)]
pub struct PermissionCreateTask {
    pub sql_1: String,
    pub param: Permission,
}

impl Default for PermissionCreateTask {
    fn default() -> Self {
        Self {
            sql_1: String::from("INSERT INTO iam_permissions (id, permission_name, permission_key) VALUES($1, $2, $3)"),
            param: Default::default(),
        }
    }
}

impl PermissionCreateTask {
    pub fn new() -> Self {
        PermissionCreateTask::default()
    }

    pub async fn process(&self, pg: &PostgresDatabase) -> TaskResult<()> {
        let mut pool = pg.pool.get().await.unwrap();
        let stmt = pool.prepare(&self.sql_1).await.unwrap();
        match pool
            .query(
                &stmt,
                &[
                    &self.param.permission_id,
                    &self.param.permission_name,
                    &self.param.permission_key,
                ],
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(TaskError::TaskUniqueConstraint),
        }
    }
}
