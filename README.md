# Rust Backend

A progressive exploration of backend development with Rust, demonstrating different approaches to building web servers and APIs.

### 1. Synchronous Implementation

The synchronous version uses Rust's standard library (`std::net::TcpListener`) to create a basic HTTP server. This implementation:

- Processes one connection at a time
- Blocks during I/O operations
- Has limited scalability for handling concurrent connections
- Handles HTTP parsing and response formatting manually

## Getting Started

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
