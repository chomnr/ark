use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::platform::iam::{permission::model::Permission, role::model::Role};

/// Represents a user's basic information.
///
/// Includes identification, contact details, and account timestamps.
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub verified: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Represents a user's authentication information with an OAuth provider.
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAuthInfo {
    pub oauth_id: String,
    pub oauth_provider: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccessInfo {
    pub role: Vec<Role>,
    pub permission: Vec<Permission>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSecurity {
    token: SecurityToken,
    stamp: String,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
// security token for the user...
pub struct SecurityToken {
    token: String,
    expiry: u64,
    action: String,
}

/// Represents a user.
///
/// Combines basic user information and authentication details.
#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    // Fields from the 'users' table
    pub info: UserInfo, // Basic user information
    // Fields from the 'user_oauth' table
    pub auth: UserAuthInfo, // Authentication details
    // Fields from 'user_roles' and 'user_permissions'
    pub access: UserAccessInfo, // Permission and role details
    // Security stamp and token used to generate reset passwords etc;
    pub security: UserSecurity,
}

impl User {
    pub fn builder() -> UserBuilder {
        UserBuilder::new()
    }
}

#[derive(Default)]
pub struct UserBuilder {
    info: UserInfo,
    auth: UserAuthInfo,
    access: UserAccessInfo,
    security: UserSecurity,
}

impl UserBuilder {
    pub fn new() -> UserBuilder {
        UserBuilder {
            info: UserInfo {
                user_id: Uuid::new_v4().to_string(),
                username: None,
                email: None,
                verified: false,
                created_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as i64,
                updated_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as i64,
            },
            auth: UserAuthInfo::default(),
            access: UserAccessInfo::default(),
            security: UserSecurity::default(),
        }
    }

    pub fn username(mut self, username: &str) -> UserBuilder {
        self.info.username = Some(String::from(username));
        self
    }

    pub fn email(mut self, email: &str) -> UserBuilder {
        self.info.email = Some(String::from(email));
        self
    }

    pub fn verified(mut self, verified: bool) -> UserBuilder {
        self.info.verified = verified;
        self
    }

    pub fn created_at(mut self, created_at: i64) -> UserBuilder {
        self.info.created_at = created_at;
        self
    }

    pub fn updated_at(mut self, updated_at: i64) -> UserBuilder {
        self.info.updated_at = updated_at;
        self
    }

    pub fn oauth_id(mut self, oauth_id: &str) -> UserBuilder {
        self.auth.oauth_id = String::from(oauth_id);
        self
    }

    pub fn oauth_provider(mut self, oauth_provider: &str) -> UserBuilder {
        self.auth.oauth_provider = String::from(oauth_provider);
        self
    }

    pub fn role(mut self, roles: Vec<Role>) -> UserBuilder {
        self.access.role = roles;
        self
    }

    pub fn permission(mut self, permissions: Vec<Permission>) -> UserBuilder {
        self.access.permission = permissions;
        self
    }

    pub fn security_stamp(mut self) -> UserBuilder {
        self.security.stamp = Uuid::new_v4().as_simple().to_string();
        self
    }

    /// Runs the builder result through the validator ensuring
    /// it meets the criteria.
    /*
    pub fn validate_and_build(self) -> ValidationResult<User> {
        UserValidator::validate(User {
            info: self.info,
            auth: self.auth,
            access: self.access
        })
    }
    */

    pub fn build(mut self) -> User {
        User {
            info: self.info,
            auth: self.auth,
            access: self.access,
            security: UserSecurity::default(),
        }
    }
}
