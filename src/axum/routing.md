# axum/routing

Reference implementation. Read after attempting the exercise.

```rust
fn create_api_router() -> axum::Router {
    use axum::{
        extract::{Path, Query},
        http::StatusCode,
        routing::{delete, get, post},
        Router,
    };
    use std::collections::HashMap;

    let user_routes = Router::new()
        .route("/users", get(|| async { "List of users" }))
        .route(
            "/users",
            post(|| async { (StatusCode::CREATED, "User created") }),
        )
        .route(
            "/users/:id",
            get(|Path(id): Path<String>| async move { format!("User: {}", id) }),
        )
        .route(
            "/users/:id",
            delete(|Path(id): Path<String>| async move { format!("User {} deleted", id) }),
        );

    let search_route = Router::new().route(
        "/search",
        get(|Query(params): Query<HashMap<String, String>>| async move {
            params
                .get("q")
                .map(|query| format!("Searching for: {}", query))
                .ok_or(StatusCode::BAD_REQUEST)
        }),
    );

    Router::new()
        .nest("/api", user_routes)
        .nest("/api", search_route)
}
```
