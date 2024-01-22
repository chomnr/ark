use crate::app::{database::redis::RedisDatabase, service::cache::INBOUND_CACHE};

use super::message::{CacheRequest, CacheStorage};

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
        self.initialize_listener(redis_clone);
    }

    /// Initializes and starts the cache listener.
    ///
    /// # Arguments
    /// - `redis_clone`: A cloned instance of `RedisDatabase` used for handling database operations.
    ///
    /// # Examples
    /// ```
    /// // Assume `pg_clone` is a cloned instance of PostgresDatabase
    /// self.initialize_listener(pg_clone);
    /// ```
    fn initialize_listener(self, redis_clone: RedisDatabase) {
        tokio::task::spawn(async move {
            let inbound_receiver = &INBOUND_CACHE.1;
            println!("[ARK] Cache initialized, now listening to incoming requests.");
            while let Ok(cache_request) = inbound_receiver.recv() {
                self.process_incoming_request(&redis_clone, cache_request).await;
            }
        });
    }

    
    /// Processes an incoming cache request.
    ///
    /// # Arguments
    /// - `redis_clone`: A reference to a cloned `RedisDatabase` used for database operations.
    /// - `cache_request`: The `CacheRequest` object representing the received cache.
    ///
    /// # Examples
    /// ```
    /// // Assume `redis_clone` is a reference to a RedisDatabase and `cache_request` is a valid TaskRequest
    /// self.process_incoming_request(&pg_clone, cache_request).await;
    /// ```
    async fn process_incoming_request(
        &self,
        redis_clone: &RedisDatabase,
        cache_request: CacheRequest,
    ) {
        println!(
            "[CACHE] Successfully received a cache from {}. Cache Storage: {:?}.",
            cache_request.cache_id, cache_request.cache_storage
        );
        self.handle_task_request(redis_clone, cache_request).await;
    }

    
    /// Handles a given task request based on its type.
    ///
    /// # Arguments
    /// - `redis`: A reference to the `RedisDatabase` used for database operations.
    /// - `cache_request`: The `CacheRequest` object containing details about the task to be handled.
    ///
    /// # Examples
    /// ```
    /// // Assume `redis` is a reference to a RedisDatabase and `cache_request` is a valid CacheRequest
    /// self.handle_task_request(&redis, cache_request).await;
    /// ```
    async fn handle_task_request(&self, redis: &RedisDatabase, task_request: CacheRequest) {
        match task_request.cache_storage {
            CacheStorage::Permission => {
                //let task_response = PermissionTaskHandler::handle(pg, task_request).await;
                //Self::send_task_response(task_response);
            },
            CacheStorage::Role => todo!(),
            CacheStorage::User => todo!(),
        }
    }
}