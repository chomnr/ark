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

use std::sync::RwLock;

use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

use self::message::{CacheRequest, CacheResponse};

//use self::{message::CacheRequest, error::CacheResult};

pub mod error;
pub mod manager;
pub mod message;
pub mod reader;

static INBOUND_CACHE: Lazy<(Sender<CacheRequest>, Receiver<CacheRequest>)> =
    Lazy::new(|| unbounded());
static OUTBOUND_CACHE: Lazy<(Sender<CacheResponse>, Receiver<CacheResponse>)> =
    Lazy::new(|| unbounded());

pub trait LocalizedCache<T> {
    fn add(perm: T);
    fn update(search_by: &str, update_for: &str, value: &str);
    fn remove(identifier: &str);
    fn get_cache() -> &'static RwLock<Vec<T>>;
}

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
