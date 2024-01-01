use oauth2::{CsrfToken, PkceCodeVerifier, PkceCodeChallenge};
use serde::{Serialize, Deserialize};

pub struct AuthIntegritySuite {
    pkce_challenge: PkceCodeChallenge,
    user: UserIntegrity
}

#[derive(Serialize, Deserialize)]
pub struct UserIntegrity {
    state: CsrfToken,
    verifier: PkceCodeVerifier,
}

impl AuthIntegritySuite {
    pub fn gen(csrf: CsrfToken) -> Self {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        Self {
            pkce_challenge,
            user: UserIntegrity {
                state: csrf,
                verifier: pkce_verifier,
            },
        }
    }
}