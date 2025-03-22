use tokio::net::TcpListener;

mod router;
use crate::router::process_request_response;

// How the server works (asynchronous with Tokio - non-blocking)
// - Handles multiple connections concurrently
// - Uses non-blocking I/O operations
// - Spawns lightweight tasks instead of threads
// - Scales efficiently to handle thousands of concurrent connections
//
// Operations are asynchronous, meaning they do not block the main thread while waiting for I/O.
// While processing a client request in a spawned task, the main loop can continue
// accepting new connections without waiting for previous clients to complete.

#[tokio::main] // Tokio runtime
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // STARTING SERVER

    // Assign port
    let port: u16 = 3_000;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| format!("Error binding to TCP port {}: {}", port, e))?;

    // When port is :0, the OS assigns port automatically
    //let listener = TcpListener::bind("127.0.0.1:0").await?;
    //let port = listener.local_addr()?.port();

    println!("Server initialized on port {}", port);

    // HANDLE INCOMING CONNECTIONS

    loop {
        // Accept connections asynchronously
        let (stream, _) = listener
            .accept()
            .await
            .map_err(|e| format!("Failed to accept connection: {}", e))?;

        // Each new connection is handled in its own asynchronous task,
        // allowing the server to continue accepting new connections
        // while processing existing ones concurrently
        tokio::spawn(async move {
            if let Err(e) = process_request_response(stream).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}
