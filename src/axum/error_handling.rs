// Error Handling with Axum
//
// Learning Objectives:
// - Implement custom error types for APIs
// - Practice error transformation and response mapping
// - Understand Result types in async handlers
//
// cargo test --bin error_handling

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

/// Implement IntoResponse for ApiError to convert errors into HTTP responses.
/// - NotFound should return 404 with JSON error response
/// - BadRequest should return 400 with JSON error response
/// - InternalError should return 500 with JSON error response
/// - Unauthorized should return 401 with JSON error response
/// 
/// Create a router with these endpoints:
/// - GET "/resource/{id}" - Returns "Resource: {id}" if id is numeric, otherwise BadRequest
/// - GET "/protected" - Always returns Unauthorized
/// - GET "/fail" - Always returns InternalError
/// - GET "/missing" - Always returns NotFound
fn create_error_handling_router() -> axum::Router {
    todo!("Implement router with custom error handling")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;
    use axum::http::Request;
    use serde_json::Value;

    #[tokio::test]
    async fn test_valid_resource() {
        let app = create_error_handling_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/resource/123")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Resource: 123");
    }

    #[tokio::test]
    async fn test_bad_request_error() {
        let app = create_error_handling_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/resource/abc")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response: Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(error_response["error"], "BadRequest");
        assert!(error_response["message"].as_str().unwrap().contains("Invalid"));
    }

    #[tokio::test]
    async fn test_unauthorized_error() {
        let app = create_error_handling_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/protected")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response: Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(error_response["error"], "Unauthorized");
        assert!(error_response["message"].as_str().unwrap().contains("authentication"));
    }

    #[tokio::test]
    async fn test_internal_error() {
        let app = create_error_handling_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/fail")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response: Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(error_response["error"], "InternalError");
        assert!(error_response["message"].as_str().unwrap().contains("internal"));
    }

    #[tokio::test]
    async fn test_not_found_error() {
        let app = create_error_handling_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/missing")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response: Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(error_response["error"], "NotFound");
        assert!(error_response["message"].as_str().unwrap().contains("not found"));
    }

    #[tokio::test]
    async fn test_error_response_structure() {
        let app = create_error_handling_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/missing")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_response: Value = serde_json::from_slice(&body).unwrap();
        
        // Verify the response has the expected structure
        assert!(error_response.get("error").is_some());
        assert!(error_response.get("message").is_some());
        assert_eq!(error_response.as_object().unwrap().len(), 2);
    }
}