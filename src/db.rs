use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::{Pool, PooledConnection};
use bb8_postgres::tokio_postgres::{Config, Error as PgError, NoTls};
use std::env;
// Arc (Atomic Reference Counting) allows safely sharing the pool between multiple threads
// It maintains a count of references and only deallocates when all references are dropped
use std::sync::Arc;
// OnceLock ensures the pool is initialized exactly once in a thread-safe manner
// Perfect for global resources that should only be created once
use std::sync::OnceLock;

// Static global variable to store the connection pool
// This is initialized once and remains available throughout the application's lifecycle
static DB_POOL: OnceLock<Arc<Pool<PostgresConnectionManager<NoTls>>>> = OnceLock::new();

/// Initializes the PostgreSQL connection pool.
/// This function should be called at application startup.
///
/// # Returns
///
/// * `Result<(), PgError>` - Success or a PostgreSQL error
pub async fn init_pool() -> Result<(), PgError> {
    // PostgreSQL connection configuration using environment variables
    // with default values if they're not defined
    let pg_config = Config::new()
        .host(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()))
        .port(env::var("DB_PORT").map_or(5432, |p| p.parse().unwrap_or(5432)))
        .dbname(env::var("DB_NAME").unwrap_or_else(|_| "test-db".to_string()))
        .user(env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()))
        .password(env::var("DB_PASSWORD").unwrap_or_else(|_| "123456".to_string()))
        .to_owned();

    // Creating the PostgreSQL connection manager with the configuration
    // NoTls indicates that TLS won't be used (unencrypted connection)
    let manager = PostgresConnectionManager::new(pg_config, NoTls);

    // Building the pool with specific configurations
    let pool = Pool::builder()
        .max_size(15) // Maximum number of connections in the pool
        .min_idle(Some(2)) // Keep at least 2 idle connections available
        .connection_timeout(std::time::Duration::from_secs(15)) // Maximum time to obtain a connection
        .idle_timeout(Some(std::time::Duration::from_secs(60 * 10))) // Maximum time a connection can remain idle
        .max_lifetime(Some(std::time::Duration::from_secs(60 * 30))) // Maximum lifetime for any connection
        .build(manager)
        .await?;

    // Wrap the pool in Arc for thread-safe sharing
    let pool = Arc::new(pool);

    // Try to set the global pool only once
    // If it's already set, ignore this attempt (protection against reinitialization)
    DB_POOL.set(pool).unwrap_or_else(|_| {
        eprintln!("Attempt to restart ignored pool");
    });

    println!("Connection to PostgreSQL established successfully");
    Ok(())
}

/// Gets a connection from the pool.
/// This function should be used every time database interaction is needed.
///
/// # Returns
///
/// * `Result<PooledConnection<'static, PostgresConnectionManager<NoTls>>, String>` - A connection
///   from the pool or an error message
pub async fn get_connection()
-> Result<PooledConnection<'static, PostgresConnectionManager<NoTls>>, String> {
    // Try to get a reference to the global pool
    // If the pool isn't initialized, return an error
    let pool = DB_POOL
        .get()
        .ok_or_else(|| "The pool is not initialized".to_string())?;

    // The 'static lifetime here indicates that the connection can exist for the entire
    // duration of the program. This is important because the pool itself is static,
    // and connections borrowed from it need to have a lifetime that can potentially
    // match the pool's lifetime.

    // Get a connection from the pool and convert any error to String
    pool.get().await.map_err(|e| e.to_string())
}
