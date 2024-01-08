use std::env;

use bb8::{Pool, PooledConnection};
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};

pub struct PostgresConfig {
    host: String,
    user: String,
    password: String,
    dbname: String,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: env::var("PG_HOST").expect("PG_HOST"),
            user: env::var("PG_USER").expect("PG_USER"),
            password: env::var("PG_PASSWORD").expect("PG_PASSWORD"),
            dbname: env::var("PG_DBNAME").expect("PG_DBNAME"),
        }
    }
}

impl PostgresConfig {
    pub fn new(host: String, user: String, password: String, dbname: String) -> Self {
        Self {
            host,
            user,
            password,
            dbname,
        }
    }

    fn to_conn_string(&self) -> String {
        format!(
            "host={} user={} password={} dbname={}",
            self.host, self.user, self.password, self.dbname
        )
    }
}

#[derive(Clone)]
pub struct PostgresDatabase {
    pub pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl PostgresDatabase {
    pub async fn new(pg_config: PostgresConfig) -> Self {
        let manager =
            PostgresConnectionManager::new_from_stringlike(pg_config.to_conn_string(), NoTls)
                .unwrap();
        Self {
            pool: Pool::builder().build(manager).await.unwrap(),
        }
    }

    pub async fn get(&self) -> PooledConnection<'_, PostgresConnectionManager<NoTls>> {
        self.pool
            .get()
            .await
            .unwrap()
    }
}