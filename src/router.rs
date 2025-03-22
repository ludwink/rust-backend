use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

// CONSTANTS FOR HTTP RESPONSE
const HTTP_OK: &str = "HTTP/1.1 200 OK\r\n";
const HTTP_NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n";
const HTTP_INTERNAL_ERROR: &str = "HTTP/1.1 500 Internal Server Error\r\n";
const CONTENT_TYPE_JSON: &str = "Content-Type: application/json\r\n";
const CONTENT_LENGTH: &str = "Content-Length: ";

// PROCESS REQUEST AND RESPONSE
pub async fn process_request_response(
    mut stream: TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a buffer using Vec<u8> instead of a fixed-size array
    // Initial capacity of 10KB, but can grow if needed
    let mut buffer = vec![0; 10240];

    // Asynchronously read the HTTP request from client
    match stream.read(&mut buffer).await {
        // Case 1: Client closed connection (0 bytes read)
        Ok(0) => {
            println!("Client closed connection before sending data");
            return Ok(());
        }

        // Case 2: Successfully read n bytes
        Ok(n) => {
            // Convert received bytes to a UTF-8 string for processing
            // Only convert the bytes that were actually read [0..n]
            let request = String::from_utf8_lossy(&buffer[0..n]).to_string();

            // Process the request and determine the appropriate response
            // - status_line: e.g., "HTTP/1.1 200 OK"
            // - content_type: e.g., "Content-Type: application/json"
            // - body: the response content (JSON, HTML, etc.)
            let (status_line, content_type, body) = process_route(&request).await;

            // Construct the complete HTTP response
            // 1. Status line (e.g., "HTTP/1.1 200 OK")
            // 2. Headers (Content-Type, Content-Length)
            // 3. Blank line (\r\n\r\n) separating headers from body
            // 4. Response body
            let response = format!(
                "{}{}{}{}\r\n\r\n{}",
                status_line,
                content_type,
                CONTENT_LENGTH,
                body.len(),
                body
            );

            // Write response asynchronously
            // Send the response bytes to the client without blocking
            stream.write_all(response.as_bytes()).await?;
            // Ensure all data is immediately sent without blocking
            stream.flush().await?;
        }

        // Case 3: Error during read operation
        Err(e) => {
            eprintln!("Error reading data from client: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}

// PROCESS ROUTES
// This function parses incoming HTTP requests and
// directs them to the appropriate handler based on method and path
//
// Example of a raw HTTP request as received in the buffer converted to String:
// ```
// POST /users HTTP/1.1
// Content-Type: application/json
// User-Agent: PostmanRuntime/7.43.2
// Accept: */*
// Postman-Token: 4c1ebe5c-248f-463e-b980-732f64f49c40
// Host: localhost:3000
// Accept-Encoding: gzip, deflate, br
// Connection: keep-alive
// Content-Length: 48
//
// {
//     "name": "Parker",
//     "status": true
// }
// ```
//
// The parsing process:
// 1. Extract the first line (e.g., "POST /users HTTP/1.1")
// 2. Split this line into parts: ["POST", "/users", "HTTP/1.1"]
// 3. Use the method (parts[0]) and path (parts[1]) to route the request
//
// For POST requests with a body:
// - The HTTP headers and body are separated by a blank line (\r\n\r\n)
// - The body contains the payload (e.g., JSON data)
// - Function `extract_body_from_request` is used to extract this content
//
async fn process_route(request: &str) -> (&'static str, &'static str, String) {
    // Extract the first line of the request (e.g., "POST /users HTTP/1.1")
    let request_line = request.lines().next().unwrap_or("");

    // Split the request line into parts
    // Example: for "POST / HTTP/1.1" -> parts = ["POST", "/", "HTTP/1.1"]
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    // Return an error if the request doesn't contain at least a method and path
    if parts.len() < 2 {
        return (
            HTTP_NOT_FOUND,
            CONTENT_TYPE_JSON,
            r#"{"error": "Invalid request"}"#.to_string(),
        );
    }

    // Extract the HTTP method (e.g., "GET", "POST")
    // and the requested path (e.g., "/users")
    let method = parts[0];
    let path = parts[1];

    // Route the request based on the method and path combination
    match (method, path) {
        // Main route - responds with a simple Hello World message
        ("GET", "/") => {
            let body = r#"{"message": "Hello World!"}"#.to_string();
            (HTTP_OK, CONTENT_TYPE_JSON, body)
        }

        // USERS ROUTES
        // GET /users - Retrieve all users
        ("GET", "/users") => handle_get_all_users().await,
        // GET /users/{id} - Retrieve a specific user by ID
        // Example: "/users/123" would extract "123" as the ID
        ("GET", path) if path.starts_with("/users/") => {
            // trim_start_matches remove all occurrences of "/users/" at the beginning
            // let id = path.trim_start_matches("/users/");

            // strip_prefix removes only the first occurrence of "/users/" and leaves the rest intact
            // For "/users/123", this would extract "123" as the ID
            let id = path.strip_prefix("/users/").unwrap_or("");

            handle_get_user(id).await
        }
        // POST /users - Create a new user
        // Extracts the JSON body from the request and passes it to the handler
        // Example body: {"name": "John", "email": "john@example.com"}
        ("POST", "/users") => {
            let body = extract_body_from_request(request);
            handle_create_user(body).await
        }

        // PRODUCT ROUTES
        // GET /products - Retrieve all products
        ("GET", "/products") => handle_get_all_products().await,

        // 404 Not Found - Route not defined
        // Returns when no matching route is found
        _ => (
            HTTP_NOT_FOUND,
            CONTENT_TYPE_JSON,
            r#"{"error": "Route not found"}"#.to_string(),
        ),
    }
}

// UTILS. EXTRACT BODY
// - The HTTP headers and body are separated by a blank line (\r\n\r\n)
// - The body contains the payload (e.g., JSON data)
fn extract_body_from_request(request: &str) -> &str {
    if let Some(body_start) = request.find("\r\n\r\n") {
        &request[body_start + 4..]
    } else {
        ""
    }
}

// USERS ROUTES
async fn handle_get_all_users() -> (&'static str, &'static str, String) {
    let body = r#"{"users": []}"#.to_string();
    (HTTP_OK, CONTENT_TYPE_JSON, body)
}

async fn handle_get_user(id: &str) -> (&'static str, &'static str, String) {
    let body = format!(
        r#"{{"id": "{}", "name": "User Name", "email": "user@example.com"}}"#,
        id
    );
    (HTTP_OK, CONTENT_TYPE_JSON, body)
}

async fn handle_create_user(body: &str) -> (&'static str, &'static str, String) {
    (HTTP_OK, CONTENT_TYPE_JSON, body.to_string())
}

// PRODUCTS ROUTES
async fn handle_get_all_products() -> (&'static str, &'static str, String) {
    let body = r#"{"error": "Internal Server Error"}"#.to_string();
    (HTTP_INTERNAL_ERROR, CONTENT_TYPE_JSON, body)
}
