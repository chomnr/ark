use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::app::platform::iam::permission::model::Permission;

use super::task::RoleCreateTask;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    pub role_id: String,
    pub role_name: String,
    pub role_permissions: Vec<Permission>
}

impl From<RoleCreateTask> for Role {
    fn from(value: RoleCreateTask) -> Self {
        Self {
            role_id: value.role_id,
            role_name: value.role_name,
            role_permissions: value.role_permissions,
        }
    }
}

impl Role {
    pub fn new(role_id: &str, role_name: &str, role_permissions: Vec<Permission>) -> Role {
        Self {
            role_id: String::from(role_id),
            role_name: String::from(role_name),
            role_permissions,
        }
    }
    pub fn builder() -> RoleBuilder {
        RoleBuilder::new()
    }
}

#[derive(Default)]
pub struct RoleBuilder {
    role_id: String,
    role_name: String,
    role_permissions: Vec<Permission>
}

impl RoleBuilder {
    pub fn new() -> RoleBuilder {
        RoleBuilder {
            role_id: Uuid::new_v4().to_string(),
            role_name: String::default(),
            role_permissions: Vec::default(),
        }
    }

    pub fn role_name(mut self, role_name: &str) -> RoleBuilder {
        self.role_name = String::from(role_name);
        self
    }

    /*
    pub fn role_permissions(mut self, permissions: Vec<Permission>) -> RoleBuilder {
        self.role_permissions = permissions;
        self
    }
    */

    pub fn build(self) -> Role {
        Role {
            role_id: self.role_id,
            role_name: self.role_name,
            role_permissions: self.role_permissions,
        }
    }
}