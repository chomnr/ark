// on site-caching
// pub struct LocalCacheManager;

// impl LocalCacheManager {//}

//LocalCacheManager<Permission>//::

// let cache = LocalCacheManager::from("ddd")
// cache.add(33);

// proof on concept new one:
// CacheManager::send_modify_request::<Permission>("permission_cache_add", perm);
// CacheManager::send_retrieval_request::<T>(id: &str) -> CacheResult<T>;
// CacheManager::send_return_request::<T>(id: &str) -> CacheResult<T>;

// CacheRequest {cache_id, cache_action, cache}

use crate::app::{database::redis::RedisDatabase, platform::iam::user::cache::UserCacheHandler, service::cache::INBOUND_CACHE};

use super::{error::{CacheError, CacheResult}, message::{CacheLocation, CacheRequest, CacheResponse, CacheStatus}, CacheHandler, OUTBOUND_CACHE};


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

    /// Sends a cache_request to the cache channel.
    ///
    /// # Arguments
    /// - `cache_request`: The `CacheRequest` object containing details about the task to be handled.       
    ///
    /// # Examples
    /// ```
    /// // Assume `redis` is a reference to a RedisDatabase and `cache_request` is a valid CacheRequest
    /// self.send(cache_request).await;
    /// ```
    pub fn send(cache_request: CacheRequest) -> CacheResponse {
        INBOUND_CACHE.0.send(cache_request.clone()).unwrap();
        Self::wait_for_cache_completion(&cache_request)
    }

    /// Sends a cache_response to the cache_response channel.
    ///
    /// # Arguments
    /// - `cache_response`: The `CacheResponse` object containing details about the task to be handled.       
    ///
    /// # Examples
    /// ```
    /// // Assume `redis` is a reference to a RedisDatabase and `cache_response` is a valid CacheRequest
    /// self.send_response(cache_request).await;
    /// ```
    fn send_response(cache_response: CacheResponse) {
        OUTBOUND_CACHE.0.send(cache_response).unwrap();
    }

    /// Process task.
    ///
    /// # Arguments
    /// - `request`: A reference to the `TaskRequest` to process.
    ///
    /// # Examples
    /// ```
    /// // Assuming `permission` is a reference to a valid Permission
    /// Self::process_permission_task(request)
    /// ```
    pub fn process_cache(request: CacheRequest) -> CacheResult<CacheStatus> {
        let cache_response = Self::send(request);
        match cache_response.cache_status {
            CacheStatus::Completed => Ok(CacheStatus::Completed),
            CacheStatus::Failed => Err(CacheError::FailedToCompleteCache),
        }
    }

    /*
    /// Notify that the cache was missed
    ///
    /// # Arguments
    /// - `source`: The source of the cache.       
    /// - `cache_key`: The key(identifier) that was used to try to retrieve the data.
    ///
    /// # Examples
    /// ```
    /// // Assume `redis` is a reference to a RedisDatabase and `cache_request` is a valid CacheRequest
    /// self.send(cache_request).await;
    /// ```
    pub fn notify_cache_hit(source: &str, cache_key: &str, task_id: &str) {
        // todo do some actual logging here...
        println!(
            "[CACHE] HIT Successfully retrieved the requested item from the cache\n - Task Id: {}\n - Cache Key: {}\n - Timestamp: {}\n - Source: {}",
            task_id,
            cache_key,
            Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            source
        );
    }

    /// Notify that the item was not located inside th ecache.
    ///
    /// # Arguments
    /// - `source`: The source of the cache.       
    /// - `cache_key`: The key(identifier) that was used to try to retrieve the data.
    ///
    /// # Examples
    /// ```
    /// // Assume `redis` is a reference to a RedisDatabase and `cache_request` is a valid CacheRequest
    /// self.send(cache_request).await;
    /// ```
    pub fn notify_cache_miss(source: &str, cache_key: &str, task_id: &str) {
        // todo do some actual logging here...
        println!(
            "[CACHE] MISS The requested item was not found in the cache.\n - Task Id: {}\n - Cache Key: {}\n - Timestamp: {}\n - Source: {}",
            task_id,
            cache_key,
            Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            source
        );
    }
    */

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
                self.process_incoming_request(&redis_clone, cache_request)
                    .await;
            }
        });
    }

    ///
    /// todo comment.
    fn wait_for_cache_completion(cache_request: &CacheRequest) -> CacheResponse {
        for cache in OUTBOUND_CACHE.1.iter(){
            if cache_request.cache_id.eq(&cache_request.cache_id) {
                Self::log_cache_outcome(&cache);
                return cache;
            }
        }
        unreachable!()
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
            "[CACHE] Successfully received a cache request from {}.",
            cache_request.cache_id
        );
        self.handle_cache_request(redis_clone, cache_request).await;
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
    /// self.handle_cache_request(&redis, cache_request).await;
    /// ```
    async fn handle_cache_request(&self, redis: &RedisDatabase, cache_request: CacheRequest) {
        match cache_request.cache_location {
            CacheLocation::User => {
                let cache_response = UserCacheHandler::handle(redis.clone(), cache_request).await;
                Self::send_response(cache_response);
            }
        }
    }

     /// Logs the outcome of a cache based on its response status.
    ///
    /// # Arguments
    /// - `cache_response`: A reference to the `CacheResponse` object whose outcome is to be logged.
    ///
    /// # Examples
    /// ```
    /// // Assuming `cache_response` is a reference to a valid CacheResponse
    /// log_task_outcome(&task_response);
    /// ```
    fn log_cache_outcome(cache_response: &CacheResponse) {
        match cache_response.cache_status {
            CacheStatus::Completed => println!(
                "[CACHE] Cache: {} successfully completed.",
                cache_response.cache_id
            ),
            CacheStatus::Failed => println!(
                "[CACHE] Cache: {} did not complete successfully. Error: {}",
                cache_response.cache_id, cache_response.cache_error[0]
            ),
        }
    }
}

//let cache = CacheManager::new();