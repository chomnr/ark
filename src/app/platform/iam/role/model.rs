use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    role_id: String,
    role_name: String
}

impl Role {
    pub fn builder() -> RoleBuilder {
        RoleBuilder::new()
    }
}

#[derive(Default)]
pub struct RoleBuilder {
    role_id: String,
    role_name: String
}

impl RoleBuilder {
    pub fn new() -> RoleBuilder {
        RoleBuilder {
            role_id: Uuid::new_v4().to_string(),
            role_name: String::default(),
        }
    }

    pub fn role_name(mut self, role_name: &str) -> RoleBuilder {
        self.role_name = String::from(role_name);
        self
    }

    pub fn build(self) -> Role {
        Role {
            role_id: self.role_id,
            role_name: self.role_name
        }
    }
}