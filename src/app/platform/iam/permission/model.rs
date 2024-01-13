use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Permission {
    permission_id: String,
    permission_name: String,
    permission_key: String,
}

impl Permission {
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
