use core::fmt;
use std::{env, sync::Arc};

use axum::{extract::FromRef, Extension, Router};
use bb8_redis::redis::cmd;
use tokio::net::TcpListener;
use tower_cookies::{CookieManagerLayer, Key};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use super::{
    adapter::oauth_adapter::OAuthCollectionAdapter,
    database::{
        postgres::{PostgresConfig, PostgresDatabase},
        redis::{RedisConfig, RedisDatabase},
    },
    platform::iam::auth::route::oauth_routes,
    services::task::manager::TaskManager,
};

static ADDRESS: &str = "0.0.0.0";
static PORT: usize = 3000;
static MODE: ServerMode = ServerMode::Development;

pub static SESSION_COOKIE_NAME: &str = "pl.session";
pub static INTEGRITY_COOKIE_NAME: &str = "pl.integrity";

/// Represents a server configuration.
///
/// This struct holds the necessary configuration details for setting up and running a server.
/// It includes information about the server's address, operational mode, and tracing status.
///
/// # Fields
///
/// * `address` - The network address of the server, represented as a `String`.
/// * `mode` - The operational mode of the server, indicated by the `ServerMode` enum.
//  * `router` - The axum router.
///
/// # Example
///
/// ```
/// let server = ArcServer {
///     address: "127.0.0.1:8080".to_string(),
///     mode: ServerMode::Development,
///     tracing: true,
/// };
/// // The server is now configured to run on localhost port 8080 in development mode with tracing enabled.
/// ```
pub struct ArkServer {
    address: String,
    port: usize,
    mode: ServerMode,
    router: Router,
}

impl ArkServer {
    pub async fn default() -> Self {
        Self {
            address: ADDRESS.to_string(),
            port: PORT,
            mode: MODE,
            router: Router::new()
                .nest("/auth/", oauth_routes())
                .layer(Extension(Arc::new(ArkState::default().await)))
                .layer(CookieManagerLayer::new()),
        }
    }
    /// Executes server operations based on the current server mode.
    ///
    /// This function checks the server mode (`self.mode`) and executes the corresponding
    /// server operation. There are three modes: Production, Development, and Maintenance.
    /// Each mode triggers a different behavior.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let arc = ArkServer::default();
    /// arc.run().await; // starts the server in self.mode mode
    /// ```
    pub async fn run(self, pg: PostgresDatabase, redis: RedisDatabase) {
        let tcp = TcpListener::bind(&self.get_addr()).await.unwrap();
        println!("[ARK] mode: {}", self.mode.to_string());
        match self.mode {
            ServerMode::Production => {}
            ServerMode::Development => {
                Self::enable_tracing();
            }
            ServerMode::Maintenance => {
                Self::enable_tracing();
            }
        }
        println!(
            "[ARK] router initialized, now listening on port {}.",
            &self.port
        );
        Self::load_prerequisites(pg.clone(), redis.clone()).await;
        Self::register_tasks(pg, redis).await;
        axum::serve(tcp, self.router).await.unwrap();
    }

    /// Retrieves the full network address of the server.
    ///
    /// This function combines the server's address and port into a single `String`
    /// representation, formatted as "address:port". It's useful for quickly obtaining
    /// the complete address endpoint of the server.
    ///
    /// # Returns
    ///
    /// A `String` representing the server's full address.
    ///
    /// # Example
    ///
    /// ```
    /// let server = ArkServer { address: "127.0.0.1".to_string(), port: 8080, ... };
    /// let address = server.get_addr();
    /// assert_eq!(address, "127.0.0.1:8080");
    /// ```
    fn get_addr(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    /// Initializes tracing functionality for the application.
    ///
    /// This function sets up the tracing subscriber with environment-based filtering and
    /// a standard format layer. It's intended to be called during the server's startup
    /// phase to enable logging and diagnostic tracing.
    ///
    /// By default, it uses the environment's filter configuration or falls back to a
    /// debug level for the `with_axum_htmx_askama` crate.
    ///
    /// # Example
    ///
    /// ```
    /// ArkServer::enable_tracing();
    /// // Tracing is now enabled and configured.
    /// ```
    fn enable_tracing() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        println!("[ARK] tracer initialized.");
    }

    /// Registers and starts listening for tasks using the specified PostgreSQL and Redis databases.
    ///
    /// # Arguments
    ///
    /// * `pg` - An instance of `PostgresDatabase` representing the connection to the PostgreSQL database.
    /// * `redis` - An instance of `RedisDatabase` representing the connection to the Redis database.
    ///
    /// # Examples
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let pg_database = PostgresDatabase::new(/* PostgreSQL connection parameters */);
    ///     let redis_database = RedisDatabase::new(/* Redis connection parameters */);
    ///     register_tasks(pg_database, redis_database).await;
    /// }
    /// ```
    async fn register_tasks(pg: PostgresDatabase, redis: RedisDatabase) {
        let task_mgr = TaskManager::with_databases(pg, redis);
        task_mgr.listen().await;
    }

    /// Asynchronously loads prerequisites using PostgreSQL and Redis databases.
    ///
    /// # Arguments
    ///
    /// * `pg` - An instance of `PostgresDatabase` for interacting with the PostgreSQL database.
    /// * `redis` - An instance of `RedisDatabase` for interacting with the Redis database.
    ///
    /// # Examples
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let pg_database = PostgresDatabase::new(/* PostgreSQL connection parameters */);
    ///     let redis_database = RedisDatabase::new(/* Redis database connection parameters */);
    ///     load_prerequisites(pg_database, redis_database).await;
    /// }
    /// ```
    async fn load_prerequisites(pg: PostgresDatabase, redis: RedisDatabase) {
        // redis setup
        let mut redis_pool = redis.pool.get().await.unwrap();
        //let _: () = cmd("HSET")
        //    .arg("user_sessions")
        //    .arg("placeholder_field")
        //    .arg("placeholder_value")
        //    .query_async(&mut *redis_pool)
        //    .await
        //    .unwrap();
        // postgres setup
    }
}

/// Defines the operational modes for a server.
///
/// Variants:
/// - `Production`: Mode indicating the server is in a live, production environment (value 0).
/// - `Development`: Mode for development and testing purposes (value 1).
/// - `Maintenance`: Indicates the server is in maintenance mode, possibly for updates or repairs (value 2).
#[derive(Clone, Copy, PartialEq)]
enum ServerMode {
    Production,
    Development,
    Maintenance,
}

impl fmt::Display for ServerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerMode::Production => write!(f, "Production"),
            ServerMode::Development => write!(f, "Development"),
            ServerMode::Maintenance => write!(f, "Maintenance"),
        }
    }
}

#[derive(Clone)]
pub struct ArkState {
    pub key: Key,
    pub postgres: PostgresDatabase,
    pub auth: OAuthCollectionAdapter,
    pub redis: RedisDatabase,
}

impl FromRef<ArkState> for Key {
    fn from_ref(state: &ArkState) -> Self {
        state.key.clone()
    }
}

impl ArkState {
    async fn default() -> Self {
        Self {
            key: ArkState::get_key(),
            postgres: PostgresDatabase::new(PostgresConfig::default()).await,
            redis: RedisDatabase::new(RedisConfig::default()).await,
            auth: OAuthCollectionAdapter::new(),
        }
    }

    pub fn get_key() -> Key {
        Key::from(
            env::var("COOKIE_ENCRYPTION_KEY")
                .expect("COOKIE_ENCRYPTION_KEY")
                .into_bytes()
                .as_slice(),
        )
    }
}

//  let user = User::builder().validate_and_build().unwrap();
//  UserManager::create_user(user);
//  let permission = Permission::builder()
//    .permission_key("permission_kedy")
//    .permission_name("permission_nadme")
//    .build();
//  PermissionManager::create_permission(permission);
