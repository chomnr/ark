use crate::app::database::redis::RedisDatabase;

pub struct CacheManager {
    redis: RedisDatabase,
}

impl CacheManager {
    pub fn new(redis: RedisDatabase) -> Self {
        Self { redis }
    }
}