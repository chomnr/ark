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

pub struct RedisDatabase {
    pool: Pool<RedisConnectionManager>,
}

impl RedisDatabase {
    pub async fn new() -> Self {
        todo!()
    }
}