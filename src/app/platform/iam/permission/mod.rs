use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::app::service::cache::LocalizedCache;

use self::model::Permission;

pub mod manager;
pub mod model;
pub mod task;

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

    fn remove(identifier: &str) {
        PERMISSION_CACHE.write().unwrap().retain_mut(|permission| {
            permission.permission_id != permission.permission_id
                || permission.permission_name != permission.permission_name
                || permission.permission_key != permission.permission_key
        })
    }

    fn get_cache() -> &'static RwLock<Vec<Permission>> {
        &PERMISSION_CACHE
    }
}
