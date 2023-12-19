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

    fn to_conn_string(self) -> String {
        format!(
            "host={} user={} password={} dbname={}",
            self.host, self.user, self.password, self.dbname
        )
    }
}

pub struct PostgresDatabase {
    manager: PostgresConnectionManager<NoTls>,
    builder: Option<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PostgresDatabase {
    pub fn new(pg_config: PostgresConfig) -> Self {
        let manager =
            PostgresConnectionManager::new_from_stringlike(pg_config.to_conn_string(), NoTls)
                .expect("Error: failed to create connection from PostgreSQL Config.");
        Self {
            manager,
            builder: None,
        }
    }

    pub async fn builder(&mut self) {
        self.builder = Some(
            Pool::builder()
                .build(self.manager.clone())
                .await
                .expect("Error: failed to build PostgreSQL pool."),
        );
    }

    pub async fn get(&self) -> PooledConnection<'_, PostgresConnectionManager<NoTls>> {
        self.builder
            .as_ref()
            .expect("Error: failed to connect to PostgreSQL pool")
            .get()
            .await
            .unwrap()
    }
}
