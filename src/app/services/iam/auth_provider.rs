use core::fmt;
use std::env;

use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, RevocationUrl,
    TokenUrl,
};

pub enum OAuthProviderType {
    DISCORD,
}

impl fmt::Display for OAuthProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OAuthProviderType::DISCORD => write!(f, "DISCORD"),
        }
    }
}

pub struct OAuthCredential {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    revoke_url: String,
}

impl OAuthCredential {
    pub fn from_env(provider_type: OAuthProviderType) -> Self {
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

pub struct OAuthProvider;
impl OAuthProvider {
    /// OAuth2 Discord Client
    pub fn get_discord() -> BasicClient {
        let provider = OAuthCredential::from_env(OAuthProviderType::DISCORD);
        let redirect_url = env::var("OAUTH2_REDIRECT_URL").expect("OAUTH2_REDIRECT_URL");
        BasicClient::new(
            ClientId::new(provider.client_id),
            Some(ClientSecret::new(provider.client_secret)),
            AuthUrl::new(provider.auth_url).unwrap(),
            Some(TokenUrl::new(provider.token_url).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
        .set_revocation_uri(RevocationUrl::new(provider.revoke_url).unwrap())
    }
}

//OAuthProvider::from("ddd");
