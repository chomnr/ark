use std::env;

use bb8::{Pool, PooledConnection};
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};

/// `PostgresConfig` - A structure representing the configuration needed to connect to a PostgreSQL database.
///
/// This struct holds the necessary parameters like host, user, password, and database name required to establish a connection
/// to a PostgreSQL database. The values for these parameters are typically expected to be provided via environment variables.
///
/// Fields:
///
/// - `host`: A `String` representing the hostname or IP address of the PostgreSQL server.
/// - `user`: The username used for authentication with the PostgreSQL server.
/// - `password`: The password corresponding to the username for authentication.
/// - `dbname`: The name of the specific database to connect to on the PostgreSQL server.
///
/// The `PostgresConfig` struct implements the `Default` trait, which allows it to be instantiated with default values
/// taken from environment variables. This implementation is particularly useful for configuring the database connection
/// in a flexible and secure manner, as it avoids hard-coding sensitive information like passwords.
///
/// The `to_connection_string` method is used to construct a PostgreSQL connection string from the stored configuration
/// parameters. This connection string can then be used to establish a connection to the database using various PostgreSQL
/// client libraries in Rust.
pub struct PostgresConfig {
    host: String,
    user: String,
    password: String,
    dbname: String,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: env::var("PG_HOST").expect("PG_HOST"),
            user: env::var("PG_USER").expect("PG_USER"),
            password: env::var("PG_PASSWORD").expect("PG_PASSWORD"),
            dbname: env::var("PG_DBNAME").expect("PG_DBNAME"),
        }
    }
}

impl PostgresConfig {
    /// Creates a new `PostgresConfig` instance with the specified parameters.
    ///
    /// This constructor allows for the manual creation of a `PostgresConfig` object by directly
    /// providing the values for the host, user, password, and database name. It is useful in scenarios
    /// where you want to construct the configuration programmatically rather than relying on environment variables.
    ///
    /// # Arguments
    ///
    /// * `host`: A `String` specifying the hostname or IP address of the PostgreSQL server.
    /// * `user`: A `String` representing the username used for authentication with the PostgreSQL server.
    /// * `password`: A `String` containing the password corresponding to the username for authentication.
    /// * `dbname`: A `String` denoting the name of the specific database to connect to on the PostgreSQL server.
    ///
    /// # Returns
    ///
    /// Returns a `PostgresConfig` instance populated with the provided database connection parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = PostgresConfig::new(
    ///     "localhost".to_string(),
    ///     "user".to_string(),
    ///     "password".to_string(),
    ///     "mydatabase".to_string(),
    /// );
    /// ```
    pub fn new(host: String, user: String, password: String, dbname: String) -> Self {
        Self {
            host,
            user,
            password,
            dbname,
        }
    }
    /// Constructs a PostgreSQL connection string from the configuration parameters.
    ///
    /// This method formats the host, user, password, and dbname into a standard connection string
    /// that is used by PostgreSQL client libraries for establishing a database connection.
    ///
    /// # Returns
    ///
    /// Returns a `String` that is the PostgreSQL connection string.
    fn to_conn_string(self) -> String {
        format!(
            "host={} user={} password={} dbname={}",
            self.host, self.user, self.password, self.dbname
        )
    }
}

/// `PostgresDatabase` - A struct representing a PostgreSQL database connection pool.
///
/// This struct encapsulates the complexity of managing a pool of connections to a PostgreSQL database.
/// It's essential for efficient database interaction in concurrent environments, where managing
/// individual connections manually would be impractical and error-prone.
///
/// Fields:
///
/// - `builder`: A `Pool<PostgresConnectionManager<NoTls>>`. This field manages the pool of connections.
///              It's crucial for maintaining efficient access to the database and handling concurrency issues.
pub struct PostgresDatabase {
    //builder: Pool<PostgresConnectionManager<NoTls>>,
    manager: PostgresConnectionManager<NoTls>,
    builder: Option<Pool<PostgresConnectionManager<NoTls>>>,
}

impl PostgresDatabase {
    /// Creates a new instance of `PostgresDatabase` using the specified PostgreSQL configuration.
    ///
    /// This method initializes the `PostgresDatabase` struct, setting up a connection manager and a connection pool.
    /// It's a critical step in ensuring that database interactions are handled efficiently and safely.
    ///
    /// # Arguments
    ///
    /// * `pg_config`: A `PostgresConfig` used to configure the PostgreSQL server connection.
    ///
    /// # Returns
    ///
    /// Returns an instance of `PostgresDatabase`. This method ensures that the database connection pool is
    /// properly configured and ready for use, which is key for any database-driven application.
    pub fn new(pg_config: PostgresConfig) -> Self {
        let manager =
            PostgresConnectionManager::new_from_stringlike(pg_config.to_conn_string(), NoTls)
                .expect("Error: failed to create connection from PostgreSQL Config.");
        Self {
            manager,
            builder: None,
        }
    }

    pub async fn builder(&mut self) {
        self.builder = Some(
            Pool::builder()
                .build(self.manager.clone())
                .await
                .expect("Error: failed to build PostgreSQL pool."),
        );
    }

    /// Retrieves a pooled database connection.
    ///
    /// This method is essential for acquiring a connection from the pool. It's designed to handle the
    /// complexities of connection management, ensuring that connections are efficiently reused and that
    /// resources are properly managed.
    ///
    /// # Returns
    ///
    /// Returns a `PooledConnection`. This method abstracts away the lower-level details of connection
    /// management, providing a simple and effective interface for database interactions.
    pub async fn get(&self) -> PooledConnection<'_, PostgresConnectionManager<NoTls>> {
        self.builder
            .as_ref()
            .expect("Error: failed to connect to PostgreSQL pool")
            .get()
            .await
            .unwrap()
    }
}
