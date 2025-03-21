use std::{
    io::{Read, Write},
    net::TcpListener,
};

// How the server works (synchronous - blocking)
// - Processes one connection at a time
// - Blocks during I/O operations
// - Has limited scalability for handling concurrent connections
// - Handles HTTP parsing and response formatting manually
//
// While reading and processing the request,
// the server blocks and cannot accept other connections until it finishes.
// After responding, the server returns to listening for the next connection.
// If a request takes too long (e.g., a slow client connection),
// the server gets blocked and cannot handle other requests until it completes.
//

fn main() {
    // STARTING SERVER

    // Assign port
    let port: u16 = 3_000;
    let listener =
        TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Error binding to TCP port {port}");

    // When port is :0, the OS assigns port automatically
    //let socket = TcpListener::bind("127.0.0.1:0").unwrap();
    //let port = socket.local_addr().unwrap().port();

    println!("Server initialized on port {}", port);

    // HANDLE INCOMING CONNECTIONS

    // For loop to iterate over each incoming connection to the server.
    // Each time a client (like a browser or curl) makes a request,
    // listener.incoming() produces a TcpStream.
    for stream in listener.incoming() {
        // Each connection attempt can either succeed (Ok(stream)) or
        // fail (Err(e)), so we use match to handle both cases.
        match stream {
            // If the connection is successful, we receive a stream object of type TcpStream.
            // This represents bidirectional communication with the client (reading and writing data).
            Ok(mut stream) => {
                // Reading the request
                // We create a buffer (byte array, [u8; n]) to read the HTTP request sent by the client.
                // stream.read(&mut buffer) reads the request data and stores it in the buffer.
                // The request arrives as text in HTTP format
                // but is actually bytes representing ASCII characters.
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

                let mut buffer = [0; 1024];
                if let Ok(_) = stream.read(&mut buffer) {
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

                    // Finally, we send the response to the client:
                    // write_all() writes the bytes to the TcpStream.
                    // flush() ensures the data is sent immediately (and the buffer is emptied).
                    stream
                        .write_all(response.as_bytes())
                        .expect("Error writing HTTP response to client");
                    stream
                        .flush()
                        .expect("Error flushing response buffer to client");
                }
            }
            Err(e) => {
                eprintln!("Error accepting TCP connection: {} ", e)
            }
        }
    }
}
