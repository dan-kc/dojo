// Custom Middleware with Axum
//
// Learning Objectives:
// - Understand middleware concepts in Axum
// - Practice request/response modification
// - Work with tower layers and services
//
// cargo test --bin middleware

/// Create a middleware that:
/// 1. Adds a "X-Request-Id" header to all responses with a unique ID
/// 2. Logs the request method and path
/// 3. Measures request duration and adds it as "X-Response-Time" header (in ms)
/// 
/// Return a Router with the middleware applied and a test route at GET "/"
/// that returns "Hello from middleware example"
fn create_router_with_middleware() -> axum::Router {
    todo!("Implement router with custom middleware for logging and headers")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;
    use axum::http::Request;

    #[tokio::test]
    async fn test_request_id_header() {
        let app = create_router_with_middleware();
        
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key("x-request-id"));
        
        let request_id = response.headers().get("x-request-id").unwrap();
        assert!(!request_id.is_empty());
    }

    #[tokio::test]
    async fn test_response_time_header() {
        let app = create_router_with_middleware();
        
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key("x-response-time"));
        
        let response_time = response.headers().get("x-response-time").unwrap();
        let time_str = response_time.to_str().unwrap();
        assert!(time_str.ends_with("ms"));
        
        // Parse the numeric part
        let time_value: f64 = time_str.trim_end_matches("ms").parse().unwrap();
        assert!(time_value >= 0.0);
    }

    #[tokio::test]
    async fn test_unique_request_ids() {
        let app = create_router_with_middleware();
        
        // Make two requests
        let app_clone = app.clone();
        let response1 = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        let response2 = app_clone
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        let id1 = response1.headers().get("x-request-id").unwrap();
        let id2 = response2.headers().get("x-request-id").unwrap();
        
        // IDs should be different
        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn test_route_content() {
        let app = create_router_with_middleware();
        
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Hello from middleware example");
    }
}