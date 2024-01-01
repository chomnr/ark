use oauth2::{CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use serde::{Serialize, Deserialize};

/// Struct to manage integrity-related aspects of authentication.
///
/// `IntegrityAuth` focuses on ensuring the security and integrity of authentication processes, particularly in scenarios involving web requests and interactions.
///
/// Fields:
/// - `state`: A `CsrfToken` used to mitigate Cross-Site Request Forgery (CSRF) attacks. It ensures that the request originates from a trusted source.
/// - `verifier`: A `PkceCodeVerifier` utilized in the Proof Key for Code Exchange (PKCE) process, enhancing the security of OAuth 2.0 flows, especially in public or untrusted clients.
///
/// This struct is typically used in contexts where additional security measures are required to protect against common web vulnerabilities during authentication.
#[derive(Serialize, Deserialize)]
pub struct IntegrityAuth {
    state: CsrfToken,
    verifier: PkceCodeVerifier,
}

pub struct IntegrityAuthSuite {
    challenge: PkceCodeChallenge,
    client: IntegrityAuth,
}

impl IntegrityAuthSuite {
    pub fn gen(csrf: CsrfToken) -> Self {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        Self {
            challenge: pkce_challenge,
            client: IntegrityAuth {
                state: csrf,
                verifier: pkce_verifier,
            },
        }
    }
}