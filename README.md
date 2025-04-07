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

You can test the server using curl:

```bash
curl http://localhost:3000
```

## 3. Manual Routing Implementation

This implementation builds upon the asynchronous Tokio version by adding a structured routing system. It features:

- A dedicated routing function to handle different HTTP methods and paths
- Support for REST API patterns (GET, POST, etc.)
- Path parameter extraction (e.g., extracting IDs from paths like `/users/123`)
- JSON request and response handling
- Organized code structure with separate handler functions for different endpoints

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

## 4. Hyper Framework Implementation

This implementation replaces manual routing with the Hyper framework, improving features as follows:

- Integration with Tokio for efficient asynchronous operations
- A structured, type-safe routing system provided by Hyper
- Enhanced handling of HTTP requests and responses
- Optimized processing of request headers and bodies
- Better performance with less custom code to maintain

### Running the Server

```shell
cargo run
```

### Testing the Server

You can test the different endpoints in the same way as in the previous versions.

## 5. Database Integration with bb8 and tokio-postgres for Asynchronous Connections

This implementation introduces asynchronous database connectivity using bb8 (a connection pool) and tokio-postgres (for non-blocking PostgreSQL operations). This allows the server to efficiently handle multiple database connections concurrently without blocking the main thread, improving scalability and performance.

Key features of this implementation:

- Uses bb8 for managing the connection pool
- Uses tokio-postgres for asynchronous database operations (e.g., SELECT, INSERT)
- Maintains the asynchronous architecture with Tokio
- Handles concurrent database connections efficiently, optimizing performance

## 6. Docker Containerization

This implementation adds Docker support, allowing the application to run in containers for easier distribution and deployment. Benefits of this approach:

- Isolated and reproducible environment for the application
- Easy dependency management
- Better portability across development, testing, and production environments
- Simple integration with CI/CD systems

### Building and Running with Docker

```shell
# Build Docker image
docker build -t rust-backend .

# Run using environment variables from .env file
docker run -d --name rust-api -p 3000:3000 --env-file .env rust-backend

# Run with explicitly specified environment variables
docker run -d --name rust-api -p 3000:3000 -e PORT=3000 -e DB_HOST=host.docker.internal -e DB_PORT=5432 -e DB_NAME=postgres -e DB_USER=postgres -e DB_PASSWORD=123456 rust-backend
```

### Useful Docker Commands

```shell
# Stop the container
docker stop rust-api

# Start the container
docker start rust-api

# View container logs
docker logs rust-api

# Remove the container
docker rm -f rust-api
```
