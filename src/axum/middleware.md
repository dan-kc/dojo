# axum/middleware

Reference implementation. Read after attempting the exercise.

```rust
fn create_router_with_middleware() -> axum::Router {
    use axum::{
        extract::Request,
        http::HeaderValue,
        middleware::{self, Next},
        response::Response,
        routing::get,
        Router,
    };
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Instant;
    static NEXT: AtomicU64 = AtomicU64::new(1);

    async fn timing_and_logging_middleware(request: Request, next: Next) -> Response {
        let start = Instant::now();
        let method = request.method().clone();
        let uri = request.uri().clone();
        let request_id = NEXT.fetch_add(1, Ordering::Relaxed).to_string();

        println!("{} {}", method, uri);

        let mut response = next.run(request).await;

        let headers = response.headers_mut();
        headers.insert("x-request-id", HeaderValue::from_str(&request_id).unwrap());

        let duration = start.elapsed();
        let duration_ms = format!("{}ms", duration.as_millis());
        headers.insert(
            "x-response-time",
            HeaderValue::from_str(&duration_ms).unwrap(),
        );

        response
    }

    Router::new()
        .route("/", get(|| async { "Hello from middleware example" }))
        .layer(middleware::from_fn(timing_and_logging_middleware))
}
```
