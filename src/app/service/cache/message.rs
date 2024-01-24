use nanoid::nanoid;
use serde::{Deserialize, Serialize};

// caches 

#[derive(Debug, Serialize, Deserialize)]
pub enum CacheLocation {
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheRequest {
    /// A unique identifier for the requested cache item.
    pub cache_id: String,

    /// The action to perform.
    pub cache_action: String,

    /// The payload to send to the cache manager.
    pub cache_payload: String,

    /// The location to store the item in the cache.
    pub cache_location: CacheLocation,
}

impl CacheRequest {
    pub fn compose_request<T: for<'a> Deserialize<'a> + Serialize>(
        cache_payload: T,
        cache_action: &str,
        cache_location: CacheLocation,
    ) -> Self {
        Self {
            cache_id: format!("cache-{}", nanoid!(7)),
            cache_payload: serde_json::to_string(&cache_payload).unwrap(),
            cache_action: String::from(cache_action),
            cache_location,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CacheResponse {
    /// The unique identifier of the cache.
    pub cache_id: String,

    /// The result of the cache . The type of this field is a String to accommodate
    /// various types of cache results.
    pub cache_result: String,

    /// The errors that occur when the task_status fails when processing the given
    /// task.
    pub cache_error: Vec<String>,
}

impl CacheResponse {
    pub fn compose_response<'a, T: Deserialize<'a> + Serialize>(
        request: CacheRequest,
        cache_result: T,
        cache_error: Vec<String>
    ) -> Self {
        Self {
            cache_id: request.cache_id,
            cache_result: serde_json::to_string(&cache_result).unwrap(),
            cache_error,
        }
    }

    pub fn throw_failed_response(request: CacheRequest, errors: Vec<String>) -> Self {
        Self {
            cache_id: request.cache_id,
            cache_result: String::default(),
            cache_error: errors,
        }
    }
}

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
//  CacheRequest::compose_request(todo!(), CacheStorage::Permission, "cache_action");
//  CacheManager::send(request);
//  CacheReader::

//CacheReader::off_site(CacheOffSite::User).read(ddd) database like redis (sends it to the cache off site channel.)
//CacheReader::on_site(CacheOnSite::Permission).read(sdadasd) it's stored in the code like a map or vec
