//! # Simple HTTP Server
//!
//! A lightweight HTTP server implemented with Hyper and Tokio.
//! Provides REST API endpoints for user and product operations.
//!
//! ## Architecture
//! The server uses Tokio's asynchronous runtime with the following characteristics:
//! - Non-blocking I/O operations
//! - Concurrent request handling via lightweight tasks (instead of threads)
//! - Efficient connection management
//!
//! Operations are asynchronous, meaning they do not block the main thread while waiting for I/O.
//! While processing a client request in a spawned task, the main loop can continue
//! accepting new connections without waiting for previous clients to complete.
//!
//! ## API Routes
//! - `GET /`: Basic greeting message
//! - `GET /users`: Retrieve all users
//! - `POST /users`: Create a new user
//! - `GET /users/{id}`: Get a specific user
//! - `GET /products`: Retrieve all products
//!
//! See the `router` module for detailed endpoint documentation.

use std::env;

use dotenvy::dotenv;
use tokio::net::TcpListener;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;

mod db;
mod router;

use db::init_pool;
use router::process_request_and_response;

/// Main entry point of the application.
///
/// Sets up an asynchronous HTTP server using Tokio and Hyper, then handles incoming
/// connections in a non-blocking manner. All request routing logic is delegated to
/// the `router` module.
///
/// # Panics
///
/// Will panic if:
/// - Unable to bind to the specified TCP port
/// - Failed to accept a connection
#[tokio::main]
async fn main() {
    // ==================== STARTING SERVER ====================

    // Load .env file
    // .ok() ignore any errors if the file does not exist (production)
    dotenv().ok();

    // Start database pool
    if let Err(e) = init_pool().await {
        eprintln!("Error starting database pool: {}", e);
        std::process::exit(1);
    }

    // Configure IP address and port for the server
    // - 0.0.0.0: Listen on all available network interfaces
    //   (allows both local and external connections)
    // - 127.0.0.1: Listen only for local connections
    // - When port is :0, the OS assigns port automatically
    let port: u16 = env::var("PORT")
        .unwrap_or("3000".to_owned())
        .parse::<u16>()
        .unwrap();
    //let addr = SocketAddr::from(([0, 0, 0, 0], 3005));
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect(&format!("Error binding to TCP port {}", port));

    println!("Server initialized on port {}", port);

    // ==================== HANDLE INCOMING CONNECTIONS ====================
    // Main loop that accepts incoming connections
    loop {
        // Wait for and accept a new connection asynchronously
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        // Adapt the TCP socket to Tokio's I/O interface
        let io = TokioIo::new(stream);

        // Each new connection is handled in its own asynchronous task,
        // allowing the server to continue accepting new connections
        // while processing existing ones concurrently
        tokio::spawn(async move {
            // Configure an HTTP service that routes requests to our handler function
            if let Err(e) = http1::Builder::new()
                .serve_connection(io, service_fn(process_request_and_response))
                .await
            {
                eprintln!("Error in HTTP connection: {}", e);
            }
        });
    }
}
