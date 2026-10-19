# axum/basic_server

Reference implementation. Read after attempting the exercise.

```rust
fn create_basic_server() -> axum::Router {
    use axum::{extract::Path, routing::get, Router};

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "OK" }))
        .route(
            "/echo/:message",
            get(|Path(message): Path<String>| async move { message }),
        )
}
```
