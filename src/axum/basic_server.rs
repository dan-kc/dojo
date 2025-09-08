// Basic HTTP Server with Axum
//
// Learning Objectives:
// - Understand basic Axum server setup
// - Practice creating simple HTTP routes
// - Work with async runtime integration
//
// cargo test --bin basic_server

/// Create a basic Axum HTTP server with the following routes:
/// - GET "/" returns "Hello, World!"
/// - GET "/health" returns "OK" with status 200
/// - GET "/echo/{message}" returns the message parameter
/// 
/// The function should return a configured Router that can be tested.
fn create_basic_server() -> axum::Router {
    todo!("Implement basic Axum server with routes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::body::Body;
    use tower::ServiceExt;
    use axum::http::Request;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_root_route() {
        let app = create_basic_server();
        
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn test_health_route() {
        let app = create_basic_server();
        
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"OK");
    }

    #[tokio::test]
    async fn test_echo_route() {
        let app = create_basic_server();
        
        let response = app
            .oneshot(Request::builder().uri("/echo/test-message").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"test-message");
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = create_basic_server();
        
        let response = app
            .oneshot(Request::builder().uri("/nonexistent").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}