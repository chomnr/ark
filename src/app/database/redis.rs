use std::env;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;

pub struct RedisConfig {
    host: String,
    user: String,
    password: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: env::var("REDIS_HOST").expect("REDIS_HOST"),
            user: env::var("REDIS_USER").expect("REDIS_USER"),
            password: env::var("REDIS_PASSWORD").expect("REDIS_PASSWORD")
        }
    }
}

impl RedisConfig {
    pub fn new(host: String, user: String, password: String) -> Self {
        Self {
            host,
            user,
            password,
        }
    }

    pub fn to_conn_string(&self) -> String {
        format!("redis://{}:{}@{}/", self.user, self.password, self.host)
    }
}

#[derive(Clone)]
pub struct RedisDatabase {
    pub pool: Pool<RedisConnectionManager>,
}

impl RedisDatabase {
    pub async fn new(redis_config: RedisConfig) -> Self {
        let manager = RedisConnectionManager::new(redis_config.to_conn_string()).unwrap();
        let pool = Pool::builder().build(manager).await.unwrap();
        Self {
            pool
        }
    }
}