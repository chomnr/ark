use std::{fmt, path::Prefix, sync::Arc};

use axum::Router;
use tokio::net::TcpListener;

use super::database::postgres::{PostgresConfig, PostgresDatabase};

pub const ARC_SERVER_ADDRESS: &str = "0.0.0.0:3000";

// `ArcResult` - A custom result type for your application.
// This type alias simplifies the usage of the standard `Result` type across your application.
// It is specialized to return `ArcError` when errors occur, providing a consistent error handling interface.
pub type ArcResult<T> = Result<T, ArcError>;

// `ArcError` - An enumeration of possible errors that can occur in your application.
// This enum provides a clear and extendable way to represent different kinds of errors.
// Adding new error types as your application grows is straightforward.
#[derive(Debug)]
pub enum ArcError {
    // Represents an error when the TCP listener fails to be created.
    // Useful for pinpointing issues related to network bindings or configurations.
    FailedToCreateTcpListener,

    // Represents an error when the HTTP server fails to run.
    // This could be due to a variety of issues, including configuration errors or runtime problems.
    FailedToRunHttpServer,
}

static PREFIX: &str = "[ARC]";

// Implementation of the `fmt::Display` trait for `ArcError`.
// This allows for user-friendly, formatted error messages.
impl fmt::Display for ArcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArcError::FailedToCreateTcpListener => {
                write!(f, "{} failed to create TCPListener", PREFIX)
            }
            ArcError::FailedToRunHttpServer => write!(f, "{} failed to run HttpServer", PREFIX),
        }
    }
}

/// `ArcServer` - A structure representing a web server configuration.
///
/// This struct encapsulates all the necessary components to configure and run a web server using the Axum framework.
/// It includes an address for the server to listen on and a router for handling HTTP requests.
///
/// Fields:
///
/// - `address`: A `&'static str` representing the network address where the server will listen for incoming connections.
///   The `'static` lifetime denotes that this string slice is valid for the duration of the program.
///
/// - `router`: An instance of `Router` which holds the route configuration. This defines the HTTP endpoints
///   the server will handle, along with their associated request handlers.
///
/// The `ArcServer` struct implements the `Default` trait, which provides a default constructor. The default
/// constructor initializes the server with a predefined address and a router.
#[derive(Clone)]
pub struct ArcServer {
    address: String,
    router: Router,
}

impl Default for ArcServer {
    fn default() -> Self {
        Self {
            address: ARC_SERVER_ADDRESS.to_owned(),
            router: Router::new(),
        }
    }
}

impl ArcServer {
    /// Asynchronously runs the server in production mode.
    ///
    /// This function binds a TCP listener to the server's address and starts serving incoming
    /// requests using the configured Axum router. It is specifically tailored for production
    /// environment usage.
    ///
    /// # Behavior
    ///
    /// - The server listens on the address specified by the `ADDRESS` constant.
    /// - It utilizes the `axum::serve` function to handle incoming HTTP requests based on the
    ///   configured routes.
    ///
    /// # Panics
    ///
    /// This function panics if binding the TCP listener fails or if an error occurs while
    /// serving requests.
    ///
    /// # Example
    ///
    /// ```
    /// let arc = ArcServer::default().await;
    /// arc.run_http_server().await;
    /// ```
    pub async fn run_http_server(self) -> ArcResult<()> {
        let tcp_listener = match TcpListener::bind(&self.address).await {
            Ok(listener) => listener,
            Err(_) => return Err(ArcError::FailedToCreateTcpListener),
        };
        println!("{} running on {}", PREFIX, self.address);
        match axum::serve(tcp_listener, self.router.into_make_service()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ArcError::FailedToRunHttpServer),
        }
    }
}

/// `ArcDatabase` - A struct representing a higher-level database abstraction.
///
/// This struct is designed to encapsulate different database connections or types.
/// Currently, it only contains a `PostgresDatabase`, but its design suggests it could be extended
/// to include other databases or additional layers of abstraction for database operations.
///
/// Fields:
///
/// - `postgres`: An instance of `PostgresDatabase`. This field is the primary connection interface
///               to a PostgreSQL database. Its purpose is to provide access to PostgreSQL-specific
///               functionality through the `PostgresDatabase` struct.
pub struct ArcDatabase {
    postgres: PostgresDatabase,
}

impl ArcDatabase {
    /// Asynchronously creates a new instance of `ArcDatabase`.
    ///
    /// This method initializes the `ArcDatabase` struct, specifically setting up its `postgres` field
    /// with a new instance of `PostgresDatabase`. It relies on the `PostgresConfig::default()` method
    /// to obtain default configuration parameters for the PostgreSQL connection.
    ///
    /// # Returns
    ///
    /// Returns an instance of `ArcDatabase`. This instance provides a unified interface for database
    /// operations, currently centered around PostgreSQL but potentially extensible to other databases.
    ///
    /// # Example
    ///
    /// ```
    /// let arc_db = ArcDatabase::new().await;
    /// ```
    ///
    /// Note: The method currently assumes a default configuration for the PostgreSQL database, which
    /// may not be suitable for all use cases. Future enhancements could include parameterizing the
    /// configuration or adding support for other database types.
    pub async fn new() -> Self {
        Self {
            postgres: PostgresDatabase::new(PostgresConfig::default()).await,
        }
    }
}
