use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use once_cell::sync::Lazy;

use crate::app::service::cache::{
    error::{CacheError, CacheResult},
    LocalizedCache,
};

use super::model::Role;

static ROLE_CACHE: Lazy<RwLock<HashMap<String, Arc<Role>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub(super) struct RoleCache;

impl LocalizedCache<Role> for RoleCache {
    fn add(item: Role) {
        let item_arc = Arc::new(item);
        let mut cache = ROLE_CACHE.write().unwrap();
        cache.insert(item_arc.role_id.clone(), Arc::clone(&item_arc));
        cache.insert(item_arc.role_name.clone(), item_arc);
    }

    fn single_add(item: Role) {
        let item_arc = Arc::new(item);
        let mut cache = ROLE_CACHE.write().unwrap();

        cache.insert(item_arc.role_id.clone(), item_arc);
    }

    fn remove(id: &str) -> CacheResult<bool> {
        let mut cache = ROLE_CACHE.write().unwrap();
        match cache.remove(id) {
            Some(_) => Ok(true),
            None => Err(CacheError::ItemNotFound),
        }
    }

    fn get(id: &str) -> CacheResult<Role> {
        let cache = ROLE_CACHE.read().unwrap();
        match cache.get(id) {
            Some(v) => Ok(v.as_ref().clone()),
            None => Err(CacheError::ItemNotFound),
        }
    }
}
