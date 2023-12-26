use bb8_postgres::tokio_postgres::Error;

use crate::app::database::postgres::PostgresDatabase;

pub struct Permission {
    id: usize,
    permission_name: String,
    permission_key: String,
}

impl Permission {
    pub fn new(id: usize, permission_name: &str, permission_key: &str) -> Self {
        Self {
            id,
            permission_name: String::from(permission_name),
            permission_key: String::from(permission_key),
        }
    }
}

pub struct PermissionManager {
    pg: PostgresDatabase,
}

impl PermissionManager {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    pub async fn create_permission(
        &self,
        permission_name: &str,
        permission_key: &str,
    ) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO permission (permission_name, permission_key) VALUES ($1, $2)")
            .await?;
        let result = pool
            .execute(&stmt, &[&permission_name, &permission_key])
            .await?;
        Ok(result)
    }

    pub async fn delete_permission(&self, permission_key: &str) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("DELETE FROM permission WHERE permission_key = $1")
            .await?;
        let result = pool.execute(&stmt, &[&permission_key]).await?;
        Ok(result)
    }

    pub async fn add_role_permission(
        &self,
        role_id: i64,
        permission_id: i64,
    ) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("INSERT INTO role_permission (role_id, permission_id) VALUES ($1, $2)")
            .await?;
        let result = pool.execute(&stmt, &[&role_id, &permission_id]).await?;
        Ok(result)
    }

    // add_role_permission()
    // remove_role_permission()

    // add_user_permission()
    // remove_user_permission()
}
