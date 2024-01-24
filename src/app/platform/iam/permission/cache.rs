use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use once_cell::sync::Lazy;

use crate::app::service::cache::{
    error::{CacheError, CacheResult},
    LocalizedCache,
};

use super::model::Permission;

// storage
static PERMISSION_CACHE: Lazy<RwLock<HashMap<String, Arc<Permission>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub(super) struct PermissionCache;

impl LocalizedCache<Permission> for PermissionCache {
    fn add(item: Permission) {
        let mut cache = PERMISSION_CACHE.write().unwrap();
        cache.insert(item.clone().permission_id, Arc::new(item.clone()));
        cache.insert(item.clone().permission_name, Arc::new(item.clone()));
        cache.insert(item.clone().permission_key, Arc::new(item.clone()));
    }

    fn remove(id: &str) -> CacheResult<bool> {
        let mut cache = PERMISSION_CACHE.write().unwrap();
        match cache.remove(id) {
            Some(_) => Ok(true),
            None => Err(CacheError::ItemNotFound),
        }
    }

    fn get(id: &str) -> CacheResult<Permission> {
        let cache = PERMISSION_CACHE.read().unwrap();
        match cache.get(id) {
            Some(v) => Ok(v.as_ref().clone()),
            None => Err(CacheError::ItemNotFound),
        }
    }
}
