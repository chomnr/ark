use bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};

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

    pub async fn create_role(&self, role_name: &str) -> IamResult<()> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO role (role_name) VALUES ($1)")
            .await
            .unwrap();
        match pool.execute(&stmt, &[&role_name]).await {
            Ok(_) => Ok(()),
            Err(_) => Err(IamError::RoleAlreadyFound),
        }
    }

    pub async fn delete_role(&self, role_name: &str) -> IamResult<()> {
        let pool = self.pg.pool.get().await.unwrap();
        let role_exists: bool = pool
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM role WHERE role_name = $1)",
                &[&role_name],
            )
            .await
            .unwrap()
            .get(0);
        if !role_exists {
            return Err(IamError::RoleCannotBeFound);
        }
        let stmt = pool
            .prepare("DELETE FROM role WHERE role_name = $1")
            .await
            .unwrap();
        match pool.execute(&stmt, &[&role_name]).await {
            Ok(_) => Ok(()),
            Err(_) => Err(IamError::RoleCannotBeFound),
        }
    }
}
