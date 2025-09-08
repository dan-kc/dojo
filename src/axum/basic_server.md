# Basic HTTP Server with Axum - Solution

```rust
async fn create_basic_server() -> axum::Router {
    use axum::{Router, routing::get, extract::Path};
    
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "OK" }))
        .route("/echo/:message", get(|Path(message): Path<String>| async move {
            message
        }))
}
```

## Explanation

This solution demonstrates the fundamentals of building an HTTP server with Axum:

1. **Router Creation**: We use `Router::new()` to create a new router instance.

2. **Basic Route**: The root route ("/") uses a simple async closure that returns a string literal.

3. **Health Check**: The "/health" endpoint follows the same pattern, returning a simple "OK" response.

4. **Path Parameters**: The "/echo/{message}" route uses `Path` extractor to capture URL segments. The `Path<String>` type automatically extracts and parses the path parameter.

5. **Async Handlers**: All handlers are async functions or closures, allowing for non-blocking I/O operations.

## Key Learning Points

- Axum uses a builder pattern for constructing routers
- Route handlers are async functions that can return any type implementing `IntoResponse`
- Path parameters are extracted using the `Path` extractor
- String literals automatically implement `IntoResponse` and return with status 200