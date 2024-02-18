//
// Cache System
//
// +------------------------+
// | System Sends Cache     |
// | Request                |
// +-----------+------------+
//             |
//             v
// +-----------+------------+       +---------------------------+
// | Inbound Channel        |       | Cache the Item            |
// | Receives Cache Request | ----> | or Check for Availability |
// +------------------------+       +-----------+---------------+
//                                           |
//                                +----------+------------+
//                                |                       |
//                        +-------v-------+       +-------v--------+
//                        | Cache         |       | Handle         |
//                        | Successfully  |       | Cache Miss     |
//                        +-------+-------+       +-------+--------+
//                                |                       |
//                                v                       v
//                        +-------+-------+       +-------+--------+
//                        | Outbound      |       | Outbound       |
//                        | Channel       |       | Channel        |
//                        | Returns Cached|       | Creates Cache  |
//                        | Item          |       | Entry & Returns|
//                        +---------------+       | Result         |
//                                                +----------------+
use axum::async_trait;
use chrono::Utc;
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

use self::{error::CacheResult, message::{CacheRequest, CacheResponse}};

pub mod error;
pub mod manager;
pub mod message;

static INBOUND_CACHE: Lazy<(Sender<CacheRequest>, Receiver<CacheRequest>)> =
   Lazy::new(|| unbounded());
static OUTBOUND_CACHE: Lazy<(Sender<CacheResponse>, Receiver<CacheResponse>)> =
    Lazy::new(|| unbounded());

pub trait LocalizedCache<T> {
    fn add(item: T);
    fn single_add(item: T);
    fn remove(id: &str) -> CacheResult<bool>;
    fn get(id: &str) -> CacheResult<T>;
}

#[async_trait]
pub trait CacheEvent<D, R, P> {
    async fn run(db: &D, request: R, param: P) -> CacheResponse;
}

/// Handles the task
#[async_trait]
pub trait CacheHandler<T> {
    async fn handle(cache_db: T, cache_request: CacheRequest) -> CacheResponse;
}

pub fn notify_cache_hit(source: &str, action: &str, task_id: &str) {
    // todo do some actual logging here...
    println!(
        "[CACHE] HIT Successfully retrieved the requested item from the cache\n - Task Id: {}\n - Cache Key: {}\n - Timestamp: {}\n - Source: {}",
        task_id,
        action,
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        source
    );
}

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

//static INBOUND_CACHE: Lazy<(Sender<CacheRequest>, Receiver<CacheRequest>)> =
//    Lazy::new(|| unbounded());
//static OUTBOUND_CACHE: Lazy<(Sender<CacheResponse>, Receiver<CacheResponse>)> =
//    Lazy::new(|| unbounded());


// Cache
// CacheItem<Permission> 

// Directional Linking?
// 

/*
pub trait LocalizedCache<T> {
    fn add(perm: T);
    fn update(search_by: &str, update_for: &str, value: &str);
    fn remove(identifier: &str);
    fn get(identifier: &str) -> CacheResult<T>;
    fn get_cache() -> &'static RwLock<Vec<T>>;
}
*/

/*
 pub search_by: String,
    pub update_for: String,
    pub value: String,
*/
/*
// these two traits are used for dealing with caching outside of our local environment

/// Used to create specific cache events ex: create update, delete etc;
#[async_trait]
pub trait CacheEvent<D, R, P> {
    async fn run(db: &D, request: R, param: P) -> CacheResponse;
}

/// Handles the task
#[async_trait]
pub trait CacheHandler<T> {
    async fn handle(cache_db: T, cache_request: CacheRequest) -> CacheResponse;
}

// end heres.
*/
