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
