use core::fmt;
use std::env;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, TokenUrl,
};

///
/// Prerequisites
///

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
    client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    revoke_url: RevocationUrl,
}

impl OAuthConfig {
    /// Creates an OAuth configuration from environment variables based on the specified provider.
    ///
    /// The function reads environment variables to set up the OAuth configuration for the given provider.
    /// It expects specific variables for client ID, client secret, authentication URL, token URL,
    /// and revocation URL to be set in the environment.
    ///
    /// `provider_type`: The type of OAuth provider for which to fetch the configuration.
    ///
    /// Example:
    /// ```
    /// let config = OAuthConfig::from_env(OAuthProvider::Discord);
    /// ```
    pub fn from_env(provider_type: OAuthProvider) -> Self {
        let name = provider_type.to_string();
        Self {
            client_id: ClientId::new(env::var(format!("{}_CLIENT_ID", name)).expect(&name)),
            client_secret: ClientSecret::new(
                env::var(format!("{}_CLIENT_SECRET", name)).expect(&name),
            ),
            auth_url: AuthUrl::new(env::var(format!("{}_AUTH_URL", name)).expect(&name))
                .expect("Error: unable to get auth url"),
            token_url: TokenUrl::new(env::var(format!("{}_TOKEN_URL", name)).expect(&name))
                .expect("Error: unable to get token url"),
            revoke_url: RevocationUrl::new(
                env::var(format!("{}_REVOCATION_URL", name)).expect(&name),
            )
            .expect("Error: unable to get revocation url"),
        }
    }
}

struct AuthClient {
    client: BasicClient,
}

impl AuthClient {
    /// Constructs a new OAuth client configuration based on the given provider.
    ///
    /// This function creates a new OAuth client with settings obtained from environment variables.
    /// The configuration includes client ID, client secret, authentication URL, token URL,
    /// and revocation URL specific to the OAuth provider.
    ///
    /// `provider`: The OAuth provider (e.g., Discord) to configure the client for.
    ///
    /// Example:
    /// ```
    /// let oau
    pub fn new(provider: OAuthProvider) -> Self {
        let config = OAuthConfig::from_env(provider);
        let redirect_url = RedirectUrl::new(env::var("OAUTH2_REDIRECT_URL").expect("REDIRECT URL"))
            .expect("Unable to get redirect url.");
        let client = BasicClient::new(
            config.client_id,
            Some(config.client_secret),
            config.auth_url,
            Some(config.token_url),
        )
        .set_redirect_uri(redirect_url)
        .set_revocation_uri(config.revoke_url);
        Self { client }
    }
}

///
/// Routing
///

/*
struct AuthClient;

impl AuthClient {
    fn login() {}
}

fn test() {}
*/
// AuthClient::login(AuthType::OAuth2, OAuth2Provider::Discord);
// AuthClient::login_standard(AuthType::Standard, None, "email", "password");
