use std::sync::RwLock;

use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::app::service::cache::{
    error::{CacheError, CacheResult},
    Cacheable,
};

use super::model::Role;

static CACHE: Lazy<RwLock<DashMap<i32, Role>>> = Lazy::new(|| RwLock::new(DashMap::new()));

impl Cacheable<Role> for Role {
    fn write(value: Role) -> CacheResult<bool> {
        let cache = CACHE.write().unwrap();
        if cache.insert(value.id, value).is_some() {
            Err(CacheError::CacheWriteFailure)
        } else {
            Ok(true)
        }
    }
    
    fn update(value: Role) -> CacheResult<bool> {
        let cache = CACHE.write().unwrap();
        if cache.contains_key(&value.id) {
            cache.insert(value.id, value).unwrap(); // No need to remove first
            Ok(true)
        } else {
            Err(CacheError::CacheUpdateFailure)
        }
    }

    fn delete(value: Role) -> CacheResult<bool> {
        let cache = CACHE.write().unwrap();
        if cache.remove(&value.id).is_some() {
            Ok(true)
        } else {
            Err(CacheError::CacheDeleteFailure)
        }
    }

    fn read(value: Role) -> CacheResult<Role> {
        let cache = CACHE.write().unwrap();
        cache
            .get(&value.id)
            .map(|v| Role::new(v.id, &v.name))
            .ok_or(CacheError::CacheReadFailure)
    }
}
