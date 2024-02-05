use serde::{Deserialize, Serialize};


#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSession {
    pub token: String,
    pub expires_in: i64,
    pub user_id: String
}

impl UserSession {
    pub fn new(token: &str, expires_in: i64, user_id: &str) -> Self {
        Self {
            token: String::from(token),
            expires_in,
            user_id: String::from(user_id)
        }
    }
}