use bb8_postgres::tokio_postgres::Error;

use crate::app::database::postgres::PostgresDatabase;

pub struct Role {
    pub id: i32,
    pub role_name: String,
}

impl Role {
    pub fn new(id: i32, role_name: &str) -> Self {
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
        let stmt = pool
            .prepare("INSERT INTO role (role_name) VALUES ($1)")
            .await?;
        let result = pool.execute(&stmt, &[&role_name]).await?;
        Ok(result)
    }

    pub async fn delete_role(&self, role_name: &str) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("DELETE FROM role WHERE role_name = $1")
            .await?;
        let result = pool.execute(&stmt, &[&role_name]).await?;
        Ok(result)
    }

    pub async fn update_role(&self, role_name: &str, new_role_name: &str) -> Result<u64, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("UPDATE role SET role_name = $1 WHERE role_name = $2")
            .await?;
        let result = pool.execute(&stmt, &[&new_role_name, &role_name]).await?;
        Ok(result)
    }

    pub async fn find_role_info(&self, role_name: &str) -> Result<Role, Error> {
        let pool = self.pg.pool.get().await.unwrap();
        let stmt = pool
            .prepare("SELECT * FROM role WHERE role_name = $1")
            .await?;
        let result = pool.query_one(&stmt, &[&role_name]).await?;
        Ok(Role { 
            id: result.get::<&str, i32>("id"), 
            role_name: result.get::<&str, String>("role_name")
        })
    }
}
