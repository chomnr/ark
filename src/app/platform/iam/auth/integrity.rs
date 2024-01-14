use oauth2::{CsrfToken, PkceCodeVerifier};
use serde::{Serialize, Deserialize};
use tower_cookies::{Cookie, cookie::time::{OffsetDateTime, Duration}};

#[derive(Serialize, Deserialize)]
pub struct UserIntegrity {
    pub csrf_token: CsrfToken,
    pub pkce_verifier: PkceCodeVerifier,
    pub provider: String,
}

impl UserIntegrity {
    pub fn new(csrf_token: CsrfToken, pkce_verifier: PkceCodeVerifier, provider: &str) -> Self {
        Self {
            csrf_token,
            pkce_verifier,
            provider: provider.to_string(),
        }
    }
    
    pub fn turn_into_cookie(self) -> Cookie<'static> {
        let mut cookie = Cookie::new("name", self.serialize());
        cookie.set_path("/");
        cookie.set_expires(OffsetDateTime::now_utc() + Duration::weeks(1));
        cookie
    }
    
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize AuthIntegrity data")
    }

    pub fn deserialize(serialized_integrity: String) -> UserIntegrity {
        let result: UserIntegrity = serde_json::from_str(&serialized_integrity.as_str())
            .expect("Failed to deserialize AuthIntegrity data");
        result
    }
}