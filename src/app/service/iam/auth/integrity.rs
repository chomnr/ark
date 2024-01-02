use oauth2::{CsrfToken, PkceCodeVerifier, PkceCodeChallenge};
use serde::{Serialize, Deserialize};
use tower_cookies::{Cookie, cookie::time::{Duration, OffsetDateTime}};

use crate::app::ark::INTEGRITY_COOKIE_NAME;

/// Represents user-specific integrity data used for authentication security.
///
/// This struct holds critical elements to ensure secure user authentication, especially in web contexts.
///
/// Attributes (derived from Serialize, Deserialize):
/// - `state`: A `CsrfToken` for mitigating Cross-Site Request Forgery (CSRF) attacks.
/// - `verifier`: A `PkceCodeVerifier` used in the Proof Key for Code Exchange (PKCE) process to enhance OAuth 2.0 security.
///
/// Methods:
/// - `to_cookie`: Converts the `UserIntegrity` instance into a secure cookie for HTTP transactions.
///   This method serializes the `UserIntegrity` data and stores it in a cookie with a set path and expiration.
///   It's typically used in web applications to maintain state and security across HTTP requests.
///
/// Example:
/// ```
/// let user_integrity = UserIntegrity {
///     state: CsrfToken::new(), // Example CSRF token creation
///     verifier: PkceCodeVerifier::new(), // Example PKCE code verifier creation
/// };
/// let cookie = UserIntegrity::to_cookie(user_integrity);
/// // The cookie can now be used in HTTP responses to maintain user integrity.
/// ```
///
/// This example demonstrates creating a `UserIntegrity` instance and converting it into a cookie, which can then be used in the context of web authentication.
#[derive(Serialize, Deserialize)]
pub struct UserIntegrity {
    pub state: CsrfToken,
    pub verifier: PkceCodeVerifier,
}

impl UserIntegrity {
    pub fn to_cookie(integrity: UserIntegrity) -> Cookie<'static> {
        let mut cookie = Cookie::new(INTEGRITY_COOKIE_NAME, serde_json::to_string(&integrity).unwrap());
        cookie.set_path("/");
        cookie.set_expires(OffsetDateTime::now_utc() + Duration::weeks(1));
        cookie
    }
}
/// Shorthand for generating PKCE Challenge
///
/// `UserIntegritySuite` combines PKCE (Proof Key for Code Exchange) challenge and user-specific integrity measures to provide a robust authentication process.
///
/// Fields:
/// - `pkce_challenge`: A `PkceCodeChallenge`, part of the PKCE flow used in OAuth 2.0, designed to secure public clients from authorization code interception attacks.
/// - `user`: A `UserIntegrity` struct holding CSRF (Cross-Site Request Forgery) protection token and a PKCE verifier to ensure the security of the user's session and authentication requests.
///
/// Methods:
/// - `gen`: A constructor function that generates a new `UserIntegritySuite` instance. It takes a `CsrfToken` and generates a new PKCE challenge-verifier pair.
///
/// Example:
/// ```
/// let csrf_token = CsrfToken::new_random() // Assume CsrfToken::new() creates a new CSRF token
/// let user_integrity_suite = UserIntegritySuite::gen(csrf_token);
/// ```
///
/// In this example, `UserIntegritySuite::gen` is used to create a new instance of `UserIntegritySuite` with a fresh PKCE challenge and CSRF token, enhancing the security of user authentication.
pub struct UserIntegritySuite {
    pub pkce_challenge: PkceCodeChallenge,
    pub user: UserIntegrity
}

impl UserIntegritySuite {
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