use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

// How the server works (asynchronous with Tokio - non-blocking)
// Use Tokio library to create a non-blocking HTTP server.
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
        let (mut stream, _) = listener
            .accept()
            .await
            .map_err(|e| format!("Failed to accept connection: {}", e))?;

        // Each new connection is handled in its own asynchronous task,
        // allowing the server to continue accepting new connections
        // while processing existing ones concurrently
        tokio::spawn(async move {
            // Read the request asynchronously
            // We create a buffer (byte array, [u8; n]) to read the HTTP request sent by the client.
            // stream.read(&mut buffer).await reads the request data asynchronously
            // and stores it in the buffer without blocking the server.
            //
            // Examples of how buffer would look after reading a request:
            // This is the text of an HTTP request, but stored as bytes in buffer.
            //
            // GET / HTTP/1.1\r\n
            // Host: 127.0.0.1:3000\r\n
            // User-Agent: curl/7.68.0\r\n
            // Accept: */*\r\n
            // \r\n
            //
            // To extract the body of the request (in case of a POST with JSON, for example)
            // We would need to read the entire request into the buffer.
            // Find the blank line \r\n\r\n that separates the headers from the body.
            // Extract the JSON content that comes after that line.
            //
            // POST / HTTP/1.1\r\n
            // Host: 127.0.0.1:3000\r\n
            // Content-Type: application/json\r\n
            // Content-Length: 18\r\n
            // \r\n
            // {"name": "Rust"}
            //

            // Buffer to read the request
            let mut buffer = [0; 1024];

            match stream.read(&mut buffer).await {
                // Client closed connection (0 bytes read)
                Ok(0) => {
                    println!("Client closed connection before sending data");
                    return;
                }

                // Successfully read n bytes
                Ok(_n) => {
                    // How we respond
                    // After reading the request (buffer), we build the HTTP response with:
                    // - Status line: "HTTP/1.1 200 OK"
                    // - Headers:
                    //    "Content-Type: application/json" (indicates we're sending JSON)
                    //    "Content-Length: {}" (length of the message body)
                    // - Body: {"res": "Hello World"} (the JSON)

                    // TEXT example response
                    // let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";

                    // JSON example response
                    let response_body = r#"{"res": "Hello World"}"#;
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                        response_body.len(),
                        response_body
                    );

                    // Write response asynchronously
                    // Finally, we send the response to the client asynchronously:
                    // write_all().await writes the bytes to the TcpStream without blocking.
                    // flush().await ensures the data is sent immediately without blocking.
                    if let Err(e) = stream.write_all(response.as_bytes()).await {
                        eprintln!("Error writing HTTP response to client: {}", e);
                        return;
                    }

                    if let Err(e) = stream.flush().await {
                        eprintln!("Error flushing response buffer to client: {}", e);
                    }
                }

                // Error handling for read failures
                Err(e) => {
                    eprintln!("Error reading data from client: {}", e);
                }
            }
        });
    }
}
