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
        let shared_item = Arc::new(item); // Create a single Arc reference to the item
        let mut cache = PERMISSION_CACHE.write().unwrap();
        cache.insert(shared_item.permission_id.clone(), Arc::clone(&shared_item));
        cache.insert(
            shared_item.permission_name.clone(),
            Arc::clone(&shared_item),
        );
        cache.insert(shared_item.permission_key.clone(), Arc::clone(&shared_item));
    }

    fn single_add(item: Permission) {
        let shared_item = Arc::new(item);
        let mut cache = PERMISSION_CACHE.write().unwrap();
        cache.insert(shared_item.permission_id.clone(), Arc::clone(&shared_item));
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
