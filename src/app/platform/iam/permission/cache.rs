use std::sync::RwLock;

use once_cell::sync::Lazy;

use super::model::Permission;

/*
pub struct PermissionCache;

static PERMISSION_CACHE: Lazy<RwLock<Vec<Permission>>> = Lazy::new(|| RwLock::new(Vec::new()));

impl LocalCache<Permission> for PermissionCache {
    fn add(item: CacheItem<Permission>) -> CacheResult<bool> {
        match PERMISSION_CACHE.write() {
            Ok(mut v) => {
                // duplicated entries should be impossible, if properly used.
                v.push(item.detail);
                return Ok(true);
            }
            Err(_) => Err(CacheError::CacheInternalError),
        }
    }

    fn update(search_by: &str, update_for: &str, value: &str) -> CacheResult<bool> {
        //
        Ok(true)
    }

    fn remove(value: &str) -> CacheResult<bool> {
        match PERMISSION_CACHE.write() {
            Ok(mut v) => {
                v.retain(|perm| perm.permission_id != value);
                return Ok(true);
            }
            Err(_) => Err(CacheError::CacheInternalError),
        }
    }
}
*/