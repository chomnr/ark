use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::task::PermissionCreateTask;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    pub permission_id: String,
    pub permission_name: String,
    pub permission_key: String,
}

impl From<PermissionCreateTask> for Permission {
    fn from(value: PermissionCreateTask) -> Self {
        Self {
            permission_id: value.permission_id,
            permission_name: value.permission_name,
            permission_key: value.permission_key,
        }
    }
}

impl Permission {
    pub fn new(permission_id: &str, permission_name: &str, permission_key: &str) -> Permission {
        Self {
            permission_id: String::from(permission_id),
            permission_name: String::from(permission_name),
            permission_key: String::from(permission_key),
        }
    }

    pub fn builder() -> PermissionBuilder {
        PermissionBuilder::new()
    }
}

#[derive(Default)]
pub struct PermissionBuilder {
    permission_id: String,
    permission_name: String,
    permission_key: String,
}

impl PermissionBuilder {
    pub fn new() -> PermissionBuilder {
        PermissionBuilder {
            permission_id: Uuid::new_v4().to_string(),
            permission_name: String::default(),
            permission_key: String::default(),
        }
    }

    pub fn permission_name(mut self, permission_name: &str) -> PermissionBuilder {
        self.permission_name = String::from(permission_name);
        self
    }

    pub fn permission_key(mut self, permission_key: &str) -> PermissionBuilder {
        self.permission_key = String::from(permission_key);
        self
    }

    pub fn build(self) -> Permission {
        Permission {
            permission_id: self.permission_id,
            permission_name: self.permission_name,
            permission_key: self.permission_key,
        }
    }
}
