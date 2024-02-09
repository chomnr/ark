use syn::token::Use;

use crate::app::service::{cache::{error::CacheResult, manager::CacheManager, message::{CacheLocation, CacheRequest, CacheResponse, CacheStatus}}, task::{error::TaskResult, manager::TaskManager, message::{TaskRequest, TaskStatus, TaskType}}};

use super::{cache::{UserAddToCache, UserReadFromCache}, model::User, task::UserCreateTask};


//use super::{task::UserCreateTask, model::User};

pub struct UserManager;

impl UserManager {
    pub fn create_user(user: User) -> TaskResult<TaskStatus> {
        let task_request = Self::create_user_request(user.clone());
        TaskManager::process_task(task_request)
        /*
        match TaskManager::process_task(task_request) {
            Ok(v) => {
                UserCacheManager::create_user_cache(user).unwrap();
                return Ok(v)
            },
            Err(er) => return Err(er),
        }
        */
    }

    fn create_user_request(user: User) -> TaskRequest {
        TaskRequest::compose_request(
            UserCreateTask {
                user
            },
            TaskType::User,
            "user_create",
        )
    }
}

pub struct UserCacheManager;

impl UserCacheManager {
    pub fn add_user_to_cache(user: User) -> CacheResult<CacheStatus> {
        let cache_request = Self::create_user_cache_request(user);
        CacheManager::process_cache(cache_request)
    }
    
    fn create_user_cache_request(user: User) -> CacheRequest {
        CacheRequest::compose_request(
            UserAddToCache {
                user
            },
            CacheLocation::User,
            "user_add_to_cache",
        )
    }

    pub fn read_user_from_cache(identifier: &str) -> CacheResult<User> {
        let cache_request = Self::read_user_cache_request(identifier);
        CacheManager::process_cache_with_result::<User>(cache_request)
    }

    fn read_user_cache_request(identifier: &str) -> CacheRequest {
        CacheRequest::compose_request(
            UserReadFromCache {
                identifier: String::from(identifier),
            },
            CacheLocation::User,
            "user_read_from_cache",
        )
    }
}
