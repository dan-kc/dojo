// Custom Extractors with Axum
//
// Learning Objectives:
// - Create custom extractors for request data
// - Understand FromRequestParts trait
// - Practice validation in extractors
//
// cargo test --bin extractors

use axum::async_trait;
use axum::extract::{FromRequestParts, Query};
use axum::http::{StatusCode, request::Parts};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ApiKey(String);

#[derive(Debug, Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

#[derive(Debug, Serialize)]
struct PaginatedResponse<T> {
    data: Vec<T>,
    page: u32,
    limit: u32,
    total: u32,
}

/// Implement a custom extractor for ApiKey that:
/// - Extracts API key from "X-API-Key" header
/// - Returns Unauthorized if header is missing
/// - Returns Forbidden if API key doesn't start with "valid-"
/// 
/// Implement a pagination extractor that:
/// - Defaults to page=1, limit=10 if not provided
/// - Max limit is 100
/// 
/// Create a router with these endpoints:
/// - GET "/protected" requires valid ApiKey and returns "Access granted: {key}"
/// - GET "/items" uses pagination and returns paginated mock data
fn create_extractor_router() -> axum::Router {
    todo!("Implement router with custom extractors for API key and pagination")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;
    use axum::http::{Request, header};
    use serde_json::Value;

    #[tokio::test]
    async fn test_api_key_valid() {
        let app = create_extractor_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/protected")
                .header("X-API-Key", "valid-secret123")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Access granted: valid-secret123");
    }

    #[tokio::test]
    async fn test_api_key_missing() {
        let app = create_extractor_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/protected")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_api_key_invalid() {
        let app = create_extractor_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/protected")
                .header("X-API-Key", "invalid-key")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_pagination_defaults() {
        let app = create_extractor_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/items")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let paginated: Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(paginated["page"], 1);
        assert_eq!(paginated["limit"], 10);
        assert_eq!(paginated["data"].as_array().unwrap().len(), 10);
    }

    #[tokio::test]
    async fn test_pagination_custom() {
        let app = create_extractor_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/items?page=2&limit=5")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let paginated: Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(paginated["page"], 2);
        assert_eq!(paginated["limit"], 5);
        assert_eq!(paginated["data"].as_array().unwrap().len(), 5);
        
        // Check that items are different from page 1
        let first_item = paginated["data"][0].as_str().unwrap();
        assert!(first_item.contains("Item 6")); // Page 2 with limit 5 should start at item 6
    }

    #[tokio::test]
    async fn test_pagination_limit_cap() {
        let app = create_extractor_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/items?limit=200")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let paginated: Value = serde_json::from_slice(&body).unwrap();
        
        // Limit should be capped at 100
        assert_eq!(paginated["limit"], 100);
    }

    #[tokio::test]
    async fn test_pagination_total_count() {
        let app = create_extractor_router();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/items")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let paginated: Value = serde_json::from_slice(&body).unwrap();
        
        // Should have a total count
        assert!(paginated["total"].as_u64().unwrap() > 0);
    }
}