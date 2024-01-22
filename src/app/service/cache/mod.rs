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
// | Receives Cache Request | ----> | and Check for Availability|
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

use crossbeam_channel::{Sender, Receiver, unbounded};
use once_cell::sync::Lazy;

use self::message::CacheRequest;

pub mod error;
pub mod manager;
pub mod message;
pub mod reader;

/// Receives items to cache.
static INBOUND_CACHE: Lazy<(Sender<CacheRequest>, Receiver<CacheRequest>)> = Lazy::new(|| unbounded());

/*
pub struct CacheItem<T> {
    pub detail: T,
}

pub trait LocalCache<T> {
    fn add(item: CacheItem<T>) -> CacheResult<bool>;
    fn update(search_by: &str, update_for: &str, value: &str) -> CacheResult<bool>;
    fn remove(value: &str) -> CacheResult<bool>;
}
*/