use crate::app::ark::ArkState;

use self::access::role::RoleManager;

pub mod identity;
pub mod access;
pub mod user;

pub struct IamManager {
    role: RoleManager,
}

impl IamManager {
    pub fn new_with_state(state: ArkState) -> Self { 
        Self {
            role: RoleManager::new(state.postgres)
        }
    }
}

pub type IamResult<T> = Result<T, IamError>;

#[derive(Debug)]
pub enum IamError {
    RoleAlreadyFound,
    RoleCannotBeFound
}

/*

pub fn test(){
    let test = IamManager::new_with_state(todo!());
}

pub struct Iam {
    pg: PostgresDatabase
}

impl Iam {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self {
            pg,
        }
    }
}
*/


/*
Iam::attach_identity(identity)
        .create_session();

    Iam::attach_identity(identity)
        .create_user();
*/
/*
use tracing::{event, Level};

use crate::app::{
    database::postgres::PostgresDatabase,
    service::iam::{account::error::UserRepositoryError, identity::model::UserIdentity},
};

use super::error::UserRepositoryResult;

static CREATE_IDENTITY_QUERY: &str =
    "INSERT INTO identity (username, email, oauth_provider, oauth_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (oauth_id)
            DO UPDATE SET last_login = CURRENT_TIMESTAMP
            RETURNING *;";

/// Repository for managing user-related data within a PostgreSQL database.
///
/// Fields:
/// - `pg`: An instance of `PostgresDatabase` representing the connection and operations specific to PostgreSQL.
pub(crate) struct UserRepository {
    pg: PostgresDatabase,
}

impl UserRepository {
    pub fn new(pg: PostgresDatabase) -> Self {
        Self { pg }
    }

    /// Asynchronously creates a new user identity in the repository.
    ///
    /// This function is responsible for adding a new `UserIdentity` record to the database.
    ///
    /// Arguments:
    /// - `identity`: Reference to a `UserIdentity` object containing the information for the new identity.
    ///
    /// Returns:
    /// - `UserRepositoryResult<()>`: A result type indicating success or failure of the operation.
    ///
    /// Note: Currently, the function body is not implemented (marked with `todo!()`).
    ///
    /// Example:
    /// ```
    /// let user_repo = UserRepository::new(database);
    /// let new_identity = UserIdentity {
    ///     id: 123,
    ///     username: "new_user".to_string(),
    ///     email: "user@example.com".to_string(),
    ///     // other fields...
    /// };
    /// let result = user_repo.create_new_identity(&new_identity).await;
    /// match result {
    ///     Ok(_) => println!("New identity created successfully"),
    ///     Err(e) => println!("Error creating new identity: {:?}", e),
    /// }
    /// ```
    pub async fn create_new_identity(&self, identity: &UserIdentity) -> UserRepositoryResult<()> {
        let client = self.pg.get().await;
        let stmt = client
            .prepare(CREATE_IDENTITY_QUERY)
            .await
            .expect("Error: failed to prepare query.");
        match client
            .execute(
                &stmt,
                &[
                    &identity.username,
                    &identity.email,
                    &identity.oauth_provider,
                    &identity.oauth_id,
                ],
            )
            .await
        {
            Ok(_) => {
                event!(
                    Level::INFO,
                    "[ARC] created a new identity for {}({})",
                    identity.username,
                    identity.id
                );
                Ok(())
            }
            Err(_) => {
                event!(
                    Level::ERROR,
                    "[ARC] failed to created an identity for {}({})",
                    identity.username,
                    identity.id
                );
                Err(UserRepositoryError::FailedToCreateIdentity)
            }
        }
    }

    //create_session
}

*/

/*

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


/*
struct Authness;

impl IamAuth {
    pub fn oauth_sign_in() {}
    pub fn oauth_callback() {}
}
*/

/*
AuthClient::new(AuthType::Discord);
*/

/*
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
*/

/*
struct AuthClient;

impl AuthClient {
    fn login() {}
}

fn test() {}
*/
// AuthClient::login(AuthType::OAuth2, OAuth2Provider::Discord);
// AuthClient::login_standard(AuthType::Standard, None, "email", "password");

*/