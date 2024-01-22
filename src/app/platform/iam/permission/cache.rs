use std::sync::RwLock;

use axum::{async_trait, http::request};
use once_cell::sync::Lazy;

/*
use crate::app::{
    database::redis::RedisDatabase,
    service::cache::{error::CacheError, message::CacheRequest, CacheHandler, CacheOnSite},
};

use super::model::Permission;

static PERMISSION_CACHE: Lazy<RwLock<Vec<Permission>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub struct PermissionCacheHandler;

#[async_trait]
impl CacheHandler for PermissionCacheHandler {
    async fn handle(redis: &RedisDatabase, cache_request: CacheRequest) {
        if cache_request.cache_action.eq("permission_cache_add") {
            //PermissionCacheAdd::run(cache_request, param);
        }

        if cache_request.cache_action.eq("permission_cache_delete") {}

        if cache_request.cache_action.eq("permission_cache_preload") {}
    }
}

pub struct PermissionCacheAdd;

#[async_trait]
impl CacheOnSite<CacheRequest, Permission> for PermissionCacheAdd {
    async fn run(request: CacheRequest, param: Permission) {
        match PERMISSION_CACHE
            .write()
            .unwrap()
            .iter_mut()
            .find(|permission| permission.permission_id == param.permission_id)
        {
            Some(permission) => {
                permission.permission_name = param.permission_name;
                permission.permission_key = param.permission_key;
            }
            None => {
                PERMISSION_CACHE.write().unwrap().push(param);
            }
        }
    }
}

*/

pub fn test() {}
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
