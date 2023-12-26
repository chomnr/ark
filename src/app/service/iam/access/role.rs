use bb8::Pool;
use bb8_postgres::{tokio_postgres::{NoTls, Error}, PostgresConnectionManager};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::iam::{IamError, IamResult},
};

pub struct Role {
    id: usize,
    role_name: String,
}

impl Role {
    pub fn new(id: usize, role_name: &str) -> Self {
        Self {
            id,
            role_name: String::from(role_name),
        }
    }
}

pub struct RoleManager {
    pg: PostgresDatabase,
}

impl RoleManager {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    pub async fn create_role(&self, role_name: &str) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool.prepare("INSERT INTO role (role_name) VALUES ($1)").await?;
        let result = pool
            .execute(&stmt, &[&role_name])
            .await?;
        Ok(result)
    }

    pub async fn delete_role(&self, role_name: &str) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool.prepare("DELETE FROM role WHERE role_name = $1").await?;
        let result = pool
            .execute(&stmt, &[&role_name])
            .await?;
        Ok(result)
    }
}
