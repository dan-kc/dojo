# Custom Middleware with Axum - Solution

```rust
async fn create_router_with_middleware() -> axum::Router {
    use axum::{
        Router,
        routing::get,
        middleware::{self, Next},
        extract::Request,
        response::Response,
        http::HeaderValue,
    };
    use std::time::Instant;
    use uuid::Uuid;
    
    async fn timing_and_logging_middleware(
        request: Request,
        next: Next,
    ) -> Response {
        let start = Instant::now();
        let method = request.method().clone();
        let uri = request.uri().clone();
        let request_id = Uuid::new_v4().to_string();
        
        // Log the request
        println!("{} {}", method, uri);
        
        // Process the request
        let mut response = next.run(request).await;
        
        // Add headers to response
        let headers = response.headers_mut();
        headers.insert(
            "x-request-id",
            HeaderValue::from_str(&request_id).unwrap()
        );
        
        let duration = start.elapsed();
        let duration_ms = format!("{}ms", duration.as_millis());
        headers.insert(
            "x-response-time",
            HeaderValue::from_str(&duration_ms).unwrap()
        );
        
        response
    }
    
    Router::new()
        .route("/", get(|| async { "Hello from middleware example" }))
        .layer(middleware::from_fn(timing_and_logging_middleware))
}
```

## Explanation

This solution demonstrates how to create custom middleware in Axum:

1. **Middleware Function**: The middleware is an async function that takes a `Request` and `Next` handler.

2. **Request Timing**: We capture the start time using `Instant::now()` and calculate duration after processing.

3. **Request ID Generation**: Using UUID (would need to add `uuid` crate) to generate unique request IDs.

4. **Request Logging**: Extracting and logging the HTTP method and URI from the request.

5. **Header Injection**: Adding custom headers to the response using `headers_mut()`.

6. **Middleware Application**: Using `layer(middleware::from_fn(...))` to apply the middleware to all routes.

## Key Learning Points

- Middleware functions receive the request and a `Next` handler to call the next layer
- Middleware can modify both requests and responses
- Headers can be added to responses using the `headers_mut()` method
- `layer()` applies middleware to all routes in the router
- Middleware execution order matters - they wrap around the actual handler