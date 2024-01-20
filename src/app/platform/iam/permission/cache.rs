use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use super::model::Permission;

static LOCAL_CACHE: Lazy<RwLock<Vec<Permission>>> = Lazy::new(|| {
    RwLock::new(Vec::new())
});

pub(super) struct PermissionCache;

// pub struct PermissionCache;
// import CacheDatabase for PermissionCache;
// import Cache<LOCAL_CACHE for PermissionCache;

pub fn test() {
    
}