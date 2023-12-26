use oauth2::CsrfToken;

pub(crate) struct UserIntegrity {
    state: String,
    csrf_token: CsrfToken
}

impl UserIntegrity {
    pub fn new(state: String, csrf_token: CsrfToken) -> Self {
        Self {
            state,
            csrf_token,
        }
    }
}