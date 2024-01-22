use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use super::error::{CacheError, CacheResult};

/* 
/// Represents the type of CacheStatus
///
/// Signal whether the cache retrieval was a hit or a miss.
pub enum CacheRetrievalStatus {
    /// Successfully found item in cache.
    Hit,
    /// Did not find item in the cache.
    Miss,
}

/// Represents the type of cache storage.
///
/// This helps the channel identify where the requested payload
/// should go.
#[derive(Debug, Serialize, Deserialize)]
pub enum CacheStorage {
    Permission,
    Role,
    User,
}

// CacheManager::add_on_site_request<T>

/// A request structure for the cache.
///
/// This struct is used to encapsulate the details of a task request.
#[derive(Serialize, Deserialize)]
pub struct CacheRequest {
    /// A unique identifier for the requested cache item.
    pub cache_id: String,

    /// What storage should the item be added to ex: permission, role, user etc;
    pub cache_storage: CacheStorage,

    /// The payload of the cache. (cache item)
    pub cache_payload: String,

    /// The desired action.
    pub cache_action: String,
}

impl CacheRequest {
    /// Composes a new task request with the given payload.
    pub fn compose_request<T: for<'a> Deserialize<'a> + Serialize>(
        cache_payload: T,
        cache_storage: CacheStorage,
        cache_action: &str,
    ) -> Self {
        Self {
            cache_id: format!("cache-{}", nanoid!(7)),
            cache_payload: serde_json::to_string(&cache_payload).unwrap(),
            cache_storage,
            cache_action: String::from(cache_action),
        }
    }

    pub fn intepret_request_payload<T: for<'a> Deserialize<'a>>(
        cache_request: &CacheRequest,
    ) -> CacheResult<T> {
        match serde_json::from_str::<T>(&cache_request.cache_payload) {
            Ok(result) => Ok(result),
            Err(_) => Err(CacheError::FailedToInterpretPayload),
        }
    }
}

*/
pub fn test() {
    //CacheRequest::compose_request(todo!(), CacheStorage::Permission, "cache_action");
    //CacheManager::send(request);
    //CacheReader::
}

//CacheReader::off_site(CacheOffSite::User).read(ddd) database like redis (sends it to the cache off site channel.)
//CacheReader::on_site(CacheOnSite::Permission).read(sdadasd) it's stored in the code like a map or vec
