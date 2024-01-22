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

use axum::async_trait;
use crossbeam_channel::{Sender, Receiver, unbounded};
use once_cell::sync::Lazy;

use crate::app::database::redis::RedisDatabase;

use self::message::CacheRequest;

//use self::{message::CacheRequest, error::CacheResult};

pub mod error;
pub mod manager;
pub mod message;
pub mod reader;

static INBOUND_CACHE: Lazy<(Sender<CacheRequest>, Receiver<CacheRequest>)> = Lazy::new(|| unbounded());
