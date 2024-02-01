pub struct UserSession {
    token: String,
    expires_in: u128
}

impl UserSession {
    pub fn new(token: &str, expires_in: u128) -> Self {
        Self {
            token: String::from(token),
            expires_in,
        }
    }
}