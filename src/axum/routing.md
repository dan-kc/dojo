# Advanced Routing with Axum - Solution

```rust
async fn create_api_router() -> axum::Router {
    use axum::{
        Router, 
        routing::{get, post, delete},
        extract::{Path, Query},
        http::StatusCode,
    };
    use std::collections::HashMap;
    
    let user_routes = Router::new()
        .route("/users", get(|| async { "List of users" }))
        .route("/users", post(|| async { 
            (StatusCode::CREATED, "User created")
        }))
        .route("/users/:id", get(|Path(id): Path<String>| async move {
            format!("User: {}", id)
        }))
        .route("/users/:id", delete(|Path(id): Path<String>| async move {
            format!("User {} deleted", id)
        }));
    
    let search_route = Router::new()
        .route("/search", get(|Query(params): Query<HashMap<String, String>>| async move {
            params.get("q")
                .map(|query| format!("Searching for: {}", query))
                .unwrap_or_else(|| "No search query provided".to_string())
        }));
    
    Router::new()
        .nest("/api", user_routes)
        .nest("/api", search_route)
}
```

## Explanation

This solution showcases advanced routing capabilities in Axum:

1. **Nested Routes**: Using `nest("/api", ...)` to group all routes under the "/api" prefix.

2. **Multiple HTTP Methods**: Same path ("/users") handles different methods (GET, POST) using `get()` and `post()`.

3. **Path Parameters**: The `:id` syntax in routes captures path segments, extracted via `Path<String>`.

4. **Query Parameters**: Using `Query<HashMap<String, String>>` to extract query string parameters.

5. **Custom Status Codes**: Returning tuples like `(StatusCode::CREATED, "message")` to set specific HTTP status codes.

## Key Learning Points

- Routes can be organized using `nest()` for cleaner API structure
- Multiple HTTP methods can be registered for the same path
- Query parameters are automatically parsed into a HashMap or custom struct
- Response tuples allow combining status codes with response bodies
- Routers can be composed by nesting multiple Router instances