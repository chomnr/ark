use std::env;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl, Scope,
    TokenUrl,
};

struct OAuthConfig {
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    auth_url: AuthUrl,
    token_url: Option<TokenUrl>,
    redirect_url: RedirectUrl,
    revocation_url: RevocationUrl,
}

#[derive(Clone)]
pub struct OAuthCollectionAdapter {
    pub discord: OAuthProvider,
}

#[derive(Clone)]
pub struct OAuthProvider {
    pub name: String,
    pub client: BasicClient,
    pub scopes: Vec<Scope>
}

#[derive(Debug)]
pub enum OAuthAdapterError {
    UnsupportedProvider,
}

impl OAuthConfig {
    /// Constructs a new `OAuthConfig` instance for a specific provider.
    ///
    /// This function dynamically builds the configuration by fetching
    /// environment variables based on the specified provider. The
    /// environment variable keys are expected to follow the format
    /// `{PROVIDER}_{PARAMETER}` (e.g., `DISCORD_CLIENT_ID`).
    ///
    /// # Arguments
    ///
    /// * `provider` - A string slice representing the name of the provider
    ///   (e.g., "discord", "google").
    ///
    /// # Panics
    ///
    /// Panics if any of the expected environment variables are not set.
    ///
    /// # Examples
    ///
    /// ```
    /// // Example of creating a new OAuthConfig for the Discord provider.
    /// // Note: This function is intended to be used within the same module,
    /// // and may not be accessible from other modules.
    /// let config = OAuthConfig::new("discord");
    /// ```
    fn new(provider: &str) -> Self {
        // Dynamic generation of environment variable keys based on the provider
        let id = format!("{0}_{1}", provider, "CLIENT_ID".to_string());
        let secret = format!("{0}_{1}", provider, "CLIENT_SECRET".to_string());
        let auth_url = format!("{0}_{1}", provider, "AUTH_URL".to_string());
        let token_url = format!("{0}_{1}", provider, "TOKEN_URL".to_string());
        let redirect_uri = "OAUTH2_REDIRECT_URL".to_string();
        let revocation_uri = format!("{0}_{1}", provider, "REVOCATION_URL".to_string());

        // Constructing the configuration structure with environment variables
        Self {
            client_id: ClientId::new(env::var(&id).expect(&id)),
            client_secret: Some(ClientSecret::new(env::var(&secret).expect(&secret))),
            auth_url: AuthUrl::new(env::var(&auth_url).expect(&auth_url)).unwrap(),
            token_url: Some(TokenUrl::new(env::var(&token_url).expect(&token_url)).unwrap()),
            redirect_url: RedirectUrl::new(env::var(&redirect_uri).expect(&redirect_uri)).unwrap(),
            revocation_url: RevocationUrl::new(env::var(&revocation_uri).expect(&revocation_uri))
                .unwrap(),
        }
    }
}

impl OAuthCollectionAdapter {
    /// Constructs a new instance containing OAuth clients for various providers.
    ///
    /// This function initializes the adapter with predefined clients. Currently,
    /// it sets up a client for Discord, but it can be extended to include other
    /// providers. Each client is configured using the `create_auth_client` method.
    ///
    /// # Examples
    ///
    /// ```
    /// // How to use OAuthCollectionAdapter to access configured OAuth clients.
    /// // This example shows accessing the Discord client.
    /// // Note: This struct is typically used within the same module.
    /// use app::adapters::oauth::OAuthCollectionAdapter;
    /// let auth = OAuthCollectionAdapter::new();
    /// let discord_client = auth.discord; // Access the Discord OAuth client
    /// ```
    pub fn new() -> Self {
        Self {
            discord: OAuthProvider {
                name: "discord".to_string(),
                client: OAuthCollectionAdapter::create_auth_client("discord"),
                scopes: OAuthCollectionAdapter::create_scopes(&["identify", "email"])
            },
        }
    }

    /// Return a provider based on its name but the provider needs to exist inside
    /// of OAuthCollectionAdapter.
    ///
    /// # Arguments
    ///
    /// * `provider` - The provider you would like to retrieve ex: discord, google etc;
    pub fn get_from<'a>(&'a self, provider: &str) -> Result<&'a OAuthProvider, OAuthAdapterError> {
        if provider.to_lowercase() == "discord" {
            return Ok(&self.discord);
        }
        Err(OAuthAdapterError::UnsupportedProvider)
    }

    /// Creates a simplified version of `BasicClient` for a specified provider.
    ///
    /// This function acts as a helper to construct `BasicClient` instances with
    /// the necessary configuration for different OAuth providers. It leverages
    /// the `OAuthConfig::new`
    ///
    /// # Arguments
    ///
    /// * `provider` - The provider you would like to use ex: discord, google etc;
    fn create_auth_client(provider: &str) -> BasicClient {
        let config = OAuthConfig::new(provider);

        BasicClient::new(
            config.client_id,
            config.client_secret,
            config.auth_url,
            config.token_url,
        )
        .set_redirect_uri(config.redirect_url)
        .set_revocation_uri(config.revocation_url)
    }

    /// A better way of adding scopes. More clean.
    ///
    /// This function acts as a helper to construct `Vec<Scope>`.
    ///
    /// # Arguments
    ///
    /// * `names` - Your desired scopes.
    fn create_scopes(names: &[&str]) -> Vec<Scope> {
        names
            .iter()
            .map(|&name| Scope::new(name.to_string()))
            .collect()
    }
}

impl std::fmt::Display for OAuthAdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OAuthAdapterError::UnsupportedProvider => {
                write!(f, "OAuthAdapterError: Unsupported OAuth Provider")
            }
        }
    }
}