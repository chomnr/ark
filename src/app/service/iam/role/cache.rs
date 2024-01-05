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
        CACHE
            .write()
            .map_err(|_| CacheError::CacheWriteFailure)?
            .insert(value.id, value)
            .map_or_else(|| Ok(true), |_| Err(CacheError::CacheWriteFailure))
    }

    fn update(value: Role) -> CacheResult<bool> {
        CACHE
            .write()
            .map_err(|_| CacheError::CacheUpdateFailure)?
            .get_mut(&value.id)
            .map(|mut entry| {
                *entry = value;
                true
            })
            .ok_or(CacheError::CacheUpdateFailure)
    }

    fn delete(value: Role) -> CacheResult<bool> {
        CACHE
            .write()
            .map_err(|_| CacheError::CacheDeleteFailure)?
            .remove(&value.id)
            .map_or_else(|| Err(CacheError::CacheDeleteFailure), |_| Ok(true))
    }

    fn read(value: Role) -> CacheResult<Role> {
        CACHE
            .read()
            .map_err(|_| CacheError::CacheReadFailure)?
            .get(&value.id)
            .map(|v| Role::new(v.id, &v.name))
            .ok_or(CacheError::CacheReadFailure)
    }
}
