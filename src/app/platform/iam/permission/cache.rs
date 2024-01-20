use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::app::service::cache::Cacheable;

use super::model::Permission;

static LOCAL_CACHE: Lazy<RwLock<Vec<Permission>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub(super) struct PermissionCache;

//PermissionCache::update("dd", "permission")

// pub struct PermissionCache;
// import CacheDatabase for PermissionCache;
// import Cache<LOCAL_CACHE for PermissionCache;
