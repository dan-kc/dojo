// Shared State Management with Axum
//
// Learning Objectives:
// - Understand application state in Axum
// - Practice using Arc for shared state
// - Work with atomic operations and mutexes
//
// cargo test --bin state_management

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Clone)]
struct AppState {
    request_count: Arc<AtomicU64>,
    visitors: Arc<Mutex<HashMap<String, u32>>>,
}

/// Create an Axum router with shared state that:
/// - Tracks total request count across all endpoints
/// - Tracks unique visitors by IP address
/// 
/// Endpoints:
/// - GET "/" increments request count and returns "Request #{count}"
/// - GET "/stats" returns JSON with total_requests and unique_visitors count
/// - POST "/visitor/{ip}" registers a visitor and returns their visit count
/// 
/// Initialize the AppState and use it in the router.
fn create_stateful_router() -> axum::Router {
    todo!("Implement router with shared application state")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;
    use axum::http::Request;
    use serde_json::Value;

    #[tokio::test]
    async fn test_request_counting() {
        let app = create_stateful_router();
        
        // First request
        let response = app.clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Request #1");
        
        // Second request
        let response = app.clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Request #2");
        
        // Third request
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Request #3");
    }

    #[tokio::test]
    async fn test_visitor_tracking() {
        let app = create_stateful_router();
        
        // Register first visit from IP
        let response = app.clone()
            .oneshot(Request::builder()
                .method("POST")
                .uri("/visitor/192.168.1.1")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Visit count: 1");
        
        // Second visit from same IP
        let response = app.clone()
            .oneshot(Request::builder()
                .method("POST")
                .uri("/visitor/192.168.1.1")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Visit count: 2");
        
        // First visit from different IP
        let response = app
            .oneshot(Request::builder()
                .method("POST")
                .uri("/visitor/10.0.0.1")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Visit count: 1");
    }

    #[tokio::test]
    async fn test_stats_endpoint() {
        let app = create_stateful_router();
        
        // Make some requests
        app.clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        app.clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        app.clone()
            .oneshot(Request::builder()
                .method("POST")
                .uri("/visitor/192.168.1.1")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        app.clone()
            .oneshot(Request::builder()
                .method("POST")
                .uri("/visitor/10.0.0.1")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        // Check stats
        let response = app
            .oneshot(Request::builder().uri("/stats").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let stats: Value = serde_json::from_slice(&body).unwrap();
        
        // Stats endpoint itself counts as a request, so we expect 5 total
        assert_eq!(stats["total_requests"], 5);
        assert_eq!(stats["unique_visitors"], 2);
    }

    #[tokio::test]
    async fn test_concurrent_state_updates() {
        let app = create_stateful_router();
        
        // Simulate concurrent requests
        let mut handles = vec![];
        
        for _ in 0..10 {
            let app_clone = app.clone();
            let handle = tokio::spawn(async move {
                app_clone
                    .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
                    .await
                    .unwrap()
            });
            handles.push(handle);
        }
        
        // Wait for all requests to complete
        for handle in handles {
            handle.await.unwrap();
        }
        
        // Check final stats
        let response = app
            .oneshot(Request::builder().uri("/stats").body(Body::empty()).unwrap())
            .await
            .unwrap();
        
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let stats: Value = serde_json::from_slice(&body).unwrap();
        
        // 10 concurrent requests + 1 stats request = 11 total
        assert_eq!(stats["total_requests"], 11);
    }
}