use std::sync::RwLock;

use once_cell::sync::Lazy;

use crate::app::service::cache::{
    error::{CacheError, CacheResult},
    LocalizedCache,
};

use self::model::Role;

use super::permission::model::Permission;

pub mod manager;
pub mod model;
pub mod task;

static ROLE_CACHE: Lazy<RwLock<Vec<Role>>> = Lazy::new(|| RwLock::new(Vec::new()));

pub struct RoleCache;

impl LocalizedCache<Role> for RoleCache {
    fn add(param: Role) {
        let mut cache = ROLE_CACHE.write().unwrap();
        match cache
            .iter_mut()
            .find(|role| role.role_id == role.role_id || role.role_name == role.role_name)
        {
            Some(role) => {
                role.role_name = param.role_name;
                role.role_permissions = param.role_permissions;
            }
            None => {
                cache.push(param);
            }
        }
    }

    fn update(search_by: &str, update_for: &str, value: &str) {
        let mut cache = ROLE_CACHE.write().unwrap();
        match cache.iter_mut().find(|permission| {
            permission.role_id.to_lowercase() == search_by.to_lowercase()
                || permission.role_name.to_lowercase() == search_by.to_lowercase()
        }) {
            Some(role) => {
                if update_for.eq_ignore_ascii_case("role_name") {
                    role.role_name = String::from(value);
                }

                if update_for.eq_ignore_ascii_case("role_permissions") {
                    // double check this one...
                    let string_to_vec: Vec<Permission> = serde_json::from_str(value).unwrap();
                    role.role_permissions = string_to_vec
                }
            }
            None => { /* Handle error. */ }
        }
    }

    fn remove(identifier: &str) {
        ROLE_CACHE.write().unwrap().retain(|permission| {
            permission.role_id != identifier && permission.role_name != identifier
        });
    }

    fn get(identifier: &str) -> CacheResult<Role> {
        match ROLE_CACHE.read().unwrap().iter().find(|permission| {
            permission.role_id == identifier || permission.role_name == identifier
        }) {
            Some(permission) => Ok(permission.clone()), // Clone the Permission here
            None => Err(CacheError::ItemNotFound),
        }
    }

    fn get_cache() -> &'static RwLock<Vec<Role>> {
        &ROLE_CACHE
    }
}
