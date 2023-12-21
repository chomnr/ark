use core::fmt;
use std::env;

/// Specifies the available authentication types for the system.
///
/// Variants:
/// - `OAuth2`: Represents the OAuth 2.0 authentication standard.
enum AuthType {
    OAuth2, // #1 priority
}

/// Enumerates supported OAuth2 providers for authentication.
///
/// Variants:
/// - `Discord`: Represents the Discord OAuth2 provider.
enum OAuthProvider {
    Discord,
}

impl fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuthProvider::Discord => write!(f, "DISCORD"),
        }
    }
}

/// Configuration settings for OAuth2 authentication.
///
/// Fields:
/// - `client_id`: The OAuth client identifier.
/// - `client_secret`: The OAuth client secret key.
/// - `auth_url`: URL for initiating the authentication process.
/// - `token_url`: URL to retrieve the authentication token.
/// - `revoke_url`: URL to revoke the authentication.
struct OAuthConfig {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    revoke_url: String,
}

impl OAuthConfig {
    pub fn from_env(provider_type: OAuthProvider) -> Self {
        let name = provider_type.to_string();
        Self {
            client_id: env::var(format!("{}_CLIENT_ID", name)).expect(&name),
            client_secret: env::var(format!("{}_CLIENT_SECRET", name)).expect(&name),
            auth_url: env::var(format!("{}_AUTH_URL", name)).expect(&name),
            token_url: env::var(format!("{}_TOKEN_URL", name)).expect(&name),
            revoke_url: env::var(format!("{}_REVOCATION_URL", name)).expect(&name),
        }
    }
}

/*
struct AuthClient;

impl AuthClient {
    fn login() {}
}

fn test() {}
*/
// AuthClient::login(AuthType::OAuth2, OAuth2Provider::Discord);
// AuthClient::login_standard(AuthType::Standard, None, "email", "password");
