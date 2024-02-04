pub struct UserSession {
    token: String,
    expires_in: u128,
    user_id: String
}

impl UserSession {
    pub fn new(token: &str, expires_in: u128, user_id: &str) -> Self {
        Self {
            token: String::from(token),
            expires_in,
            user_id: String::from(user_id)
        }
    }
}