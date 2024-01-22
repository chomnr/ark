use serde::{Serialize, Deserialize};

/// Represents the type of CacheStatus
/// 
/// Signal whether the cache retrieval was a hit or a miss.
pub enum CacheRetrievalStatus {
    /// Successfully found item in cache.
    Hit,
    /// Did not find item in the cache.
    Miss
}

/// Represents the type of cache storage.
/// 
/// This helps the channel identify where the requested payload
/// should go.
#[derive(Debug, Serialize, Deserialize)]
pub enum CacheStorage {
    Permission,
    Role,
    User
}

/// A request structure for the cache.
///
/// This struct is used to encapsulate the details of a task request.v  
#[derive(Serialize, Deserialize)]
pub struct CacheRequest {
    /// A unique identifier for the requested cache item.
    pub cache_id: String,

    /// What storage should the item be added to ex: permission, role, user etc;
    pub cache_storage: CacheStorage,

    /// The payload of the cache. (cache item)
    pub cache_payload: String
}

//CacheReader::off_site(CacheOffSite::User).read(ddd) database like redis (sends it to the cache off site channel.)
//CacheReader::on_site(CacheOnSite::Permission).read(sdadasd) it's stored in the code like a map or vec