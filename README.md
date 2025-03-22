# Rust Backend

A progressive exploration of backend development with Rust, demonstrating different approaches to building web servers and APIs.

## 1. Synchronous Implementation

The synchronous version uses Rust's standard library (`std::net::TcpListener`) to create a basic HTTP server. This implementation:

- Processes one connection at a time
- Blocks during I/O operations
- Has limited scalability for handling concurrent connections
- Handles HTTP parsing and response formatting manually

### Running the Server

```bash
cargo run
```

### Testing the Server

Once running, you can test the server using curl:

```bash
curl http://localhost:3000
```

Expected response:

```json
{ "res": "Hello World" }
```

## 2. Asynchronous Implementation with Tokio

The asynchronous version uses Tokio library to create a non-blocking HTTP server. This implementation:

- Handles multiple connections concurrently
- Uses non-blocking I/O operations
- Spawns lightweight tasks instead of threads
- Scales efficiently to handle thousands of concurrent connections
- Still handles HTTP parsing and response formatting manually
- Represents a more production-ready approach

### Running the Server

```bash
cargo run
```

### Testing the Server

Once running, you can test the server using curl:

```bash
curl http://localhost:3000
```

Expected response:

```json
{ "res": "Hello World" }
```

## 3. Manual Routing Implementation

This implementation builds upon the asynchronous Tokio version by adding a structured routing system. It features:

- A dedicated routing function to handle different HTTP methods and paths
- Support for REST API patterns (GET, POST, etc.)
- Path parameter extraction (e.g., extracting IDs from paths like `/users/123`)
- JSON request and response handling
- Organized code structure with separate handler functions for different endpoints
- Still uses manual HTTP parsing but with more sophisticated request processing

### Running the Server

```bash
cargo run
```

### Testing the Server

Once running, you can test various endpoints:

```bash
# Get the main page
curl http://localhost:3000/

# Get all users
curl http://localhost:3000/users

# Get a specific user
curl http://localhost:3000/users/123

# Create a new user
curl -X POST http://localhost:3000/users -H "Content-Type: application/json" -d '{"name": "Rust"}'

# Get all products
curl http://localhost:3000/products
```
