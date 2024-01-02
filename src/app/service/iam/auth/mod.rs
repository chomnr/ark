use serde::{Serialize, Deserialize};

use crate::app::adapter::oauth::OAuthCollectionAdapter;

pub mod integrity;

pub struct Auth {
    provider: String
}

#[derive(Serialize, Deserialize)]
pub struct AuthInfo {
    user_id: i64,
    oauth_id: String,
    provider: String
}

impl Auth {
    pub fn new(provider: &str) {
        
    }
}
