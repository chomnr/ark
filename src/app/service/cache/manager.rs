use crate::app::database::redis::RedisDatabase;

pub struct CacheManager {
    redis: RedisDatabase,
}

impl CacheManager {
    pub fn new(redis: RedisDatabase) -> Self {
        Self { redis }
    }

    /// Starts the listening process for cache requests.
    ///
    /// # Examples
    /// ```
    /// // Assuming `self` is an instance of the containing struct with a valid `redis` field
    /// self.listen();
    /// ```
    pub fn listen(self) {
        let redis_clone = self.redis.clone();
        //self.initialize_listener(pg_clone);
    }
}