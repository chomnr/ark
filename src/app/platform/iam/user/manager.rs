use syn::token::Use;

use crate::app::service::{
    cache::{
        error::CacheResult,
        manager::CacheManager,
        message::{CacheLocation, CacheRequest, CacheStatus},
    },
    task::{
        error::TaskResult,
        manager::TaskManager,
        message::{TaskRequest, TaskStatus, TaskType},
    },
};

use super::{
    cache::{UserAddToCache, UserReadFromCache},
    model::User,
    task::{UserCreateTask, UserReadTask},
};

//use super::{task::UserCreateTask, model::User};

pub struct UserManager;

impl UserManager {
    /// Create a user
    ///
    /// # Arguments
    /// - `user`: information about the user to create.
    ///
    /// # Examples
    /// ```
    /// create_user(user);
    /// ```
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

    /// Create a user request
    ///
    /// # Arguments
    /// - `user`: request for a user to be created .
    ///
    /// # Examples
    /// ```
    /// create_user_request(user);
    /// ```
    fn create_user_request(user: User) -> TaskRequest {
        TaskRequest::compose_request(UserCreateTask { user }, TaskType::User, "user_create")
    }

    /// Retrieve information about a specific user by their uui
    ///
    /// # Arguments
    /// - `identifier`: the uuid of the user.
    ///
    /// # Examples
    /// ```
    /// get_user("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ```
    pub fn get_user(identifier: &str) -> TaskResult<User> {
        let task_request = Self::get_user_request(&String::from(identifier));
        TaskManager::process_task_with_result::<User>(task_request)
    }

    /// get user request
    ///
    /// # Arguments
    /// - `identifier`: create a request for a user to be retrieved.
    ///
    /// # Examples
    /// ```
    /// get_user_request("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ```
    fn get_user_request(identifier: &str) -> TaskRequest {
        TaskRequest::compose_request(
            UserReadTask {
                identifier: String::from(identifier),
            },
            TaskType::User,
            "user_read",
        )
    }
}

pub(super) struct UserCacheManager;

impl UserCacheManager {
    /// Add user to cache.
    ///
    /// # Arguments
    /// - `user`: Add the specified user to a cache..
    ///
    /// # Examples
    /// ```
    /// add_user_to_cache(user);
    /// ```
    pub fn add_user_to_cache(user: User) -> CacheResult<CacheStatus> {
        let cache_request = Self::create_user_cache_request(user);
        CacheManager::process_cache(cache_request)
    }

    /// Add user to cache request.
    ///
    /// # Arguments
    /// - `user`: Add the specified user to a cache. 
    ///
    /// # Examples
    /// ```
    /// create_user_cache_request(user);
    /// ```
    fn create_user_cache_request(user: User) -> CacheRequest {
        CacheRequest::compose_request(
            UserAddToCache { user },
            CacheLocation::User,
            "user_add_to_cache",
        )
    }

    /// Read user from cache.
    ///
    /// # Arguments
    /// - `identifier`: Reads the specified user from the cache via uuid.
    ///
    /// # Examples
    /// ```
    /// read_user_from_cache("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ``
    pub fn read_user_from_cache(identifier: &str) -> CacheResult<User> {
        let cache_request = Self::read_user_cache_request(identifier);
        CacheManager::process_cache_with_result::<User>(cache_request)
    }

    /// Read user from cache request.
    ///
    /// # Arguments
    /// - `identifier`: Reads the specified user from the cache via uuid.
    ///
    /// # Examples
    /// ```
    /// read_user_cache_request("2f4afce2-ec56-429a-96b1-480c0b20943a");
    /// ``
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
