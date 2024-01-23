use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::app::service::cache::{LocalizedCache, error::{CacheResult, CacheError}};

use self::model::Permission;

pub mod manager;
pub mod model;
pub mod task;




/*
// Cache
static PERMISSION_CACHE: Lazy<RwLock<Vec<Permission>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub struct PermissionCache;

impl LocalizedCache<Permission> for PermissionCache {
    fn add(param: Permission) {
        let mut cache = PERMISSION_CACHE.write().unwrap();
        match cache.iter_mut().find(|permission| {
            permission.permission_id == param.permission_id
                || permission.permission_name == param.permission_name
                || permission.permission_key == param.permission_key
        }) {
            Some(permission) => {
                permission.permission_name = param.permission_name;
                permission.permission_key = param.permission_key;
            }
            None => {
                cache.push(param);
            }
        }
    }

    fn update(search_by: &str, update_for: &str, value: &str) {
        let mut cache = PERMISSION_CACHE.write().unwrap();
        match cache.iter_mut().find(|permission| {
            permission.permission_id.to_lowercase() == search_by.to_lowercase()
                || permission.permission_name.to_lowercase() == search_by.to_lowercase()
                || permission.permission_key.to_lowercase() == search_by.to_lowercase()
        }) {
            Some(permission) => {
                if update_for.eq_ignore_ascii_case("permission_name") {
                    permission.permission_name = String::from(value);
                }

                if update_for.eq_ignore_ascii_case("permission_key") {
                    permission.permission_key = String::from(value)
                }
            }
            None => {/* Handle error. */}
        }
    }

    fn remove(identifier: &str) {
        PERMISSION_CACHE.write().unwrap().retain(|permission| {
            permission.permission_id != identifier
                && permission.permission_name != identifier
                && permission.permission_key != identifier
        });
    }

    fn get(identifier: &str) -> CacheResult<Permission> {
        match PERMISSION_CACHE.read().unwrap().iter().find(|permission| {
            permission.permission_id == identifier
                || permission.permission_name == identifier
                || permission.permission_key == identifier
        }) {
            Some(permission) => Ok(permission.clone()), // Clone the Permission here
            None => Err(CacheError::ItemNotFound),
        }
    }

    fn get_cache() -> &'static RwLock<Vec<Permission>> {
        &PERMISSION_CACHE
    }
}
*/