use std::sync::RwLock;

use once_cell::sync::Lazy;

use self::model::Permission;

pub mod manager;
pub mod model;
pub mod task;

// Cache Integration...
static LOCAL_CACHE: Lazy<RwLock<Vec<Permission>>> = Lazy::new(|| RwLock::new(Vec::new()));
