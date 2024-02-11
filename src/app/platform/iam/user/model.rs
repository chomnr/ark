use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Represents a user's basic information.
///
/// Includes identification, contact details, and account timestamps.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub verified: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Represents a user's authentication information with an OAuth provider.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAuthInfo {
    pub oauth_id: String,
    pub oauth_provider: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAccessInfo {
    pub role: Vec<String>,
    pub permission: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserSecurity {
    pub token: Option<SecurityToken>,
    pub stamp: Option<String>,
}

impl UserSecurity {
    pub fn new(token: Option<SecurityToken>, stamp: Option<String>) -> UserSecurity {
        UserSecurity { token, stamp }
    }

    pub fn create(action: &str) -> UserSecurity {
        let new_stamp = Self::generate_security_stamp();
        let full = SecurityToken::new(new_stamp.clone(), action);
        UserSecurity::new(Some(full), Some(new_stamp))
    }

    pub fn generate_security_stamp() -> String {
        let mut hasher = Sha256::new();
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        hasher.update(time.to_string());
        let no_hex_stamp = hasher.finalize();
        let hex_stamp = hex::encode(no_hex_stamp);
        hex_stamp
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// security token for the user...
pub struct SecurityToken {
    pub token: String,
    pub expiry: u128,
    pub action: String,
}

impl SecurityToken {
    /// Creates a new `SecurityToken` instance.
    ///
    /// # Arguments
    /// - `token`: The security token string.
    /// - `action`: The action this token is associated with.
    ///
    /// # Returns
    /// A new `SecurityToken` instance.
    pub fn new(security_stamp: String, action: &str) -> SecurityToken {
        Self {
            token: Self::generate_token(security_stamp, action),
            expiry: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                + 900000,
            action: String::from(action),
        }
    }

    /// Deserialize a security token
    pub fn decode_then_deserialize(security_token: Option<String>) -> Option<SecurityToken> {
        if security_token.is_none() {
            return None;
        }
        let decode_data = hex::decode(security_token.unwrap());
        let decode_data_result = match decode_data {
            Ok(data) => data,
            Err(_) => return None,
        };
        let from_utf8_data = String::from_utf8(decode_data_result);
        let from_utf8_data_result = match from_utf8_data {
            Ok(data) => data,
            Err(_) => return None,
        };
        match serde_json::from_str::<SecurityToken>(&from_utf8_data_result) {
            Ok(data) => Some(data),
            Err(_) => return None,
        }
    }

    /// Serializes then encodes the serialization via hex.
    pub fn serialize_then_hex(self) -> String {
        let data = serde_json::to_string(&self).unwrap();
        hex::encode(data)
    }

    /// Creates a new 'token' used for urls.
    ///
    /// # Arguments
    /// - `security_stamp`: The security_stamp.
    /// - `action`: The action this token is associated with.
    ///
    /// Returns string of token
    fn generate_token(security_stamp: String, action: &str) -> String {
        let mut hasher = Sha256::new();
        let key_f = format!("{}:{}", security_stamp, action);
        hasher.update(key_f);
        let no_hex_token = hasher.finalize();
        let hex_token = hex::encode(no_hex_token);
        hex_token
    }


    /*
    /// Creates a new `SecurityToken` instance.
    ///
    /// # Arguments
    /// - `token`: The security token string.
    /// - `action`: The action this token is associated with.
    ///
    /// # Returns
    /// A new `SecurityToken` instance.
    pub fn new(action: &str) -> SecurityToken {
        let mut sha = Sha256::new();
        let format_key = format!("{}:{}:{}", 1, 2, 3);
        let pre_key = sha.update(format_key);
        let key = sha.finalize();
        SecurityToken {
            token: token.to_string(),
            expiry: Self::get_expiration_time(),
            action: action.to_string(),
        }
    }

    /// Calculates the expiration time for the token.
    ///
    /// # Returns
    /// The expiration time as u128 representing the number of milliseconds since UNIX EPOCH.
    fn get_expiration_time() -> u128 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => {
                let fifteen_minutes = Duration::from_secs(15 * 60); // 15 minutes in seconds
                let expiration_time = n + fifteen_minutes;
                expiration_time.as_millis() as u128
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }

    /// Get the current time in miliseconds for security stamps..
    pub fn get_current_time() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u128
    }

    /// Encodes the `SecurityToken`.
    ///
    /// # Arguments
    /// - `security_token`: The `SecurityToken` to be serialized and encoded.
    ///
    /// # Returns
    /// The encrypted token as a `String`.
    pub fn serialize_and_encode(security_token: SecurityToken) -> String {
        let serialized_token = serde_json::to_string(&security_token).unwrap();
        base64::encode(serialized_token)
    }

    pub fn deserialize_and_decode(base_64_serialized: Option<&str>) -> Option<SecurityToken> {
        if base_64_serialized == None {
            return None;
        }
        let decoded_bytes = match decode(base_64_serialized.unwrap()) {
            Ok(bytes) => bytes,
            Err(_) => return Some(SecurityToken::default()), // Handle decode error, returning a default SecurityToken
        };

        match from_slice(&decoded_bytes) {
            Ok(token) => token,
            Err(_) => Some(SecurityToken::default()), // Handle deserialize error, returning a default SecurityToken
        }
    }
    */
}

/// Represents a user.
///
/// Combines basic user information and authentication details.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn new(
        user_id: String,
        username: Option<String>,
        email: Option<String>,
        verified: bool,
        created_at: i64,
        updated_at: i64,
        oauth_id: String,
        oauth_provider: String,
        roles: Vec<String>,
        permissions: Vec<String>,
        security: UserSecurity,
    ) -> Self {
        User {
            info: UserInfo {
                user_id,
                username,
                email,
                verified,
                created_at,
                updated_at,
            },
            auth: UserAuthInfo {
                oauth_id,
                oauth_provider,
            },
            access: UserAccessInfo {
                role: roles,
                permission: permissions,
            },
            security,
        }
    }
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
                    .as_millis() as i64,
                updated_at: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
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

    pub fn role(mut self, roles: Vec<String>) -> UserBuilder {
        self.access.role = roles;
        self
    }

    pub fn permission(mut self, permissions: Vec<String>) -> UserBuilder {
        self.access.permission = permissions;
        self
    }

    pub fn security_stamp(mut self) -> UserBuilder {
        self.security.stamp = Some(Uuid::new_v4().as_simple().to_string());
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

    pub fn build(self) -> User {
        User {
            info: self.info,
            auth: self.auth,
            access: self.access,
            security: UserSecurity::default(),
        }
    }
}
