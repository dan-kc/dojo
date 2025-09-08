// WebSocket Implementation with Axum
//
// Learning Objectives:
// - Implement WebSocket connections in Axum
// - Handle bi-directional communication
// - Practice message framing and protocols
//
// cargo test --bin websockets

use axum::extract::ws::{WebSocket, WebSocketUpgrade, Message};
use axum::response::Response;
use futures::{sink::SinkExt, stream::StreamExt};

/// Create a WebSocket echo server that:
/// - Accepts WebSocket connections at "/ws"
/// - Echoes back any text message with "Echo: " prefix
/// - Responds to "ping" with "pong"
/// - Closes connection on "close" message
/// - Handles binary messages by returning their size as text
/// 
/// Also include a regular HTTP endpoint at "/" that returns "WebSocket server ready"
fn create_websocket_server() -> axum::Router {
    todo!("Implement WebSocket server with echo functionality")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;
    use axum::http::{Request, StatusCode};
    use tokio_tungstenite::tungstenite;

    #[tokio::test]
    async fn test_http_endpoint() {
        let app = create_websocket_server();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"WebSocket server ready");
    }

    #[tokio::test]
    async fn test_websocket_upgrade() {
        let app = create_websocket_server();
        
        // Test that /ws endpoint accepts WebSocket upgrade
        let response = app
            .oneshot(Request::builder()
                .uri("/ws")
                .header("Connection", "Upgrade")
                .header("Upgrade", "websocket")
                .header("Sec-WebSocket-Version", "13")
                .header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);
    }

    // Note: Full WebSocket testing would require a WebSocket client library
    // These tests verify the basic structure and upgrade mechanism
    // In a real implementation, you'd test the actual WebSocket behavior
    // with a proper WebSocket client

    #[tokio::test]
    async fn test_websocket_handler_exists() {
        // This test ensures the WebSocket handler is properly structured
        // by attempting to create the server
        let app = create_websocket_server();
        
        // If we can create the app and it responds to regular HTTP,
        // the basic structure is correct
        let response = app
            .oneshot(Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test] 
    async fn test_websocket_endpoint_without_upgrade() {
        let app = create_websocket_server();
        
        // Regular GET request to /ws without upgrade headers should fail
        let response = app
            .oneshot(Request::builder()
                .uri("/ws")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        // Should return 426 Upgrade Required or 400 Bad Request
        assert!(response.status() == StatusCode::UPGRADE_REQUIRED 
             || response.status() == StatusCode::BAD_REQUEST);
    }
}