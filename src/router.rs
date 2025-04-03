use std::{convert::Infallible, vec};

use http_body_util::BodyExt;
use hyper::{
    Method, Request, Response, StatusCode,
    body::{Buf, Incoming},
    header::CONTENT_TYPE,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Processes incoming HTTP requests and routes them to the appropriate handler.
///
/// This function serves as the main router for the HTTP server, examining the request
/// method and path to determine which handler should process the request.
///
/// # Arguments
///
/// * `req` - The incoming HTTP request to be processed
///
/// # Returns
///
/// A `Result` containing the HTTP response or an `Infallible` error type
/// (which means the function will never return an error).
///
/// # Implemented Routes
///
/// - `GET /`: Basic greeting message
/// - `GET /users`: List all users (currently returns empty list)
/// - `POST /users`: Create a new user with JSON data
/// - `GET /users/{id}`: Get information for a specific user
/// - `GET /products`: Get all products (currently returns a mock error)
///
/// # Examples
///
/// All routes return JSON responses except for the root path.
pub async fn process_request_and_response(
    req: Request<Incoming>,
) -> Result<Response<String>, Infallible> {
    let res = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Response::new("Hello World".to_owned()),
        (&Method::GET, "/users") => handle_get_all_users().await,
        (&Method::POST, "/users") => handle_create_user(req).await,
        (&Method::GET, path) if path.starts_with("/users/") => handle_get_user(req).await,
        (&Method::GET, "/products") => handle_get_all_products().await,
        _ => json_response(StatusCode::NOT_FOUND, json!({"message": "Not found"})),
    };

    Ok(res)
}

// ==================== UTILITY FUNCTIONS ====================

/// Creates a JSON HTTP response with the specified status code and body.
///
/// # Arguments
///
/// * `status` - The HTTP status code for the response
/// * `body` - The data to be serialized as JSON in the response body
///
/// # Returns
///
/// A fully formed HTTP response with the specified status and JSON body
///
/// # Panics
///
/// Will panic if:
/// - The body cannot be serialized to JSON
/// - The response cannot be built
fn json_response<T: Serialize>(status: StatusCode, body: T) -> Response<String> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .unwrap()
}

// ==================== USER ROUTES ====================
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
}

/// Handles GET requests to retrieve all users.
///
/// # Route
///
/// `GET /users`
///
/// # Response
///
/// Returns a 200 OK response with an empty array of users.
async fn handle_get_all_users() -> Response<String> {
    let users: Vec<User> = vec![];

    json_response(StatusCode::OK, users)
}

/// Handles GET requests to retrieve a specific user by ID.
///
/// # Route
///
/// `GET /users/{id}` where `{id}` must be a positive integer (u32)
///
/// # Response
///
/// - 200 OK with user data if the ID is valid
/// - 400 Bad Request if the ID is not a valid u32
async fn handle_get_user(req: Request<Incoming>) -> Response<String> {
    // Extract and validate the ID from the URL
    let last_segment = req.uri().path().split("/").last().unwrap_or("default");
    let _id: u32 = match last_segment.parse::<u32>() {
        Ok(id) => id,
        Err(_) => {
            return json_response(StatusCode::BAD_REQUEST, json!({"error": "ID must be u32"}));
        }
    };

    // Example response with mock data
    let user = User {
        name: "User A".to_owned(),
        age: 25,
    };

    json_response(StatusCode::OK, user)
}

/// Handles POST requests to create a new user.
///
/// # Route
///
/// `POST /users`
///
/// # Request Body
/// Any valid JSON data
///
/// # Response
///
/// - 200 OK with the parsed JSON if valid
/// - 400 Bad Request if the JSON is malformed or body collection fails
async fn handle_create_user(req: Request<Incoming>) -> Response<String> {
    // whole_body is basically a buffer containing all the data from the request body.
    // Collect all fragments of the request body into a single buffer
    // The HTTP body may arrive in multiple parts that need to be aggregated
    let whole_body = match req.collect().await {
        // aggregate() combines all the chunks into a single buffer.
        Ok(collected) => collected.aggregate(),
        Err(_) => {
            return json_response(
                StatusCode::BAD_REQUEST,
                json!({"error": "Failed to collect the request body"}),
            );
        }
    };

    // Attempt to parse the JSON body
    // chunk() returns a reference to the bytes in the buffer
    match serde_json::from_slice::<Value>(whole_body.chunk()) {
        Ok(json) => json_response(StatusCode::OK, json),
        Err(_) => json_response(
            StatusCode::BAD_REQUEST,
            json!({"error": "Invalid user data"}),
        ),
    }
}

// ==================== PRODUCT ROUTES ====================

/// Handles GET requests to retrieve all products.
///
/// # Route
///
/// `GET /products`
///
/// # Response
///
/// Currently returns a 500 Internal Server Error response as a placeholder.
async fn handle_get_all_products() -> Response<String> {
    json_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"error": "Internal Server Error"}),
    )
}
