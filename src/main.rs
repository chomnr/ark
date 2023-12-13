use axum::Router;
use tokio::net::TcpListener;

pub mod app;

const ARC_SERVER_ADDRESS: &str = "0.0.0.0:3000";

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
struct ArcServer {
    address: &'static str,
    router: Router,
}

impl Default for ArcServer {
    fn default() -> Self {
        Self {
            address: ARC_SERVER_ADDRESS,
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
    /// let server_config = ArcServer::default().await;
    /// server_config.run_production().await;
    /// ```
    pub async fn run_production(self) {
        let tcp_listener = TcpListener::bind(self.address)
            .await
            .expect("Something went wrong with the TcpListener");
        axum::serve(tcp_listener, self.router.into_make_service())
            .await
            .unwrap();
    }
}

#[derive(Clone)]
pub struct AppState;

#[tokio::main]
async fn main() {
    //let databse = ArcDatabase::new();
    //database.redis.pool;
    let arc = ArcServer::default();
    //setup tarpc
    //setup redis
    //setup postgres
    //schema generator
    //master server has /auth/login and /auth/callback.
    //thats it.
    println!("Hello, woddrld!");
}
