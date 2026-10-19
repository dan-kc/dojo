# axum/websockets

Reference implementation. Read after attempting the exercise.

```rust
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use futures::SinkExt;
fn create_websocket_server() -> axum::Router {
    use axum::{response::Html, routing::get, Router};

    async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(handle_socket)
    }

    async fn handle_socket(mut socket: WebSocket) {
        while let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(text) => {
                        if text == "ping" {
                            if socket
                                .send(Message::Text("pong".to_string()))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        } else if text == "close" {
                            let _ = socket.close().await;
                            break;
                        } else {
                            let response = format!("Echo: {}", text);
                            if socket.send(Message::Text(response)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Message::Binary(data) => {
                        let response = format!("{} bytes", data.len());
                        if socket.send(Message::Text(response)).await.is_err() {
                            break;
                        }
                    }
                    Message::Close(_) => {
                        break;
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
    }

    async fn root_handler() -> &'static str {
        "WebSocket server ready"
    }

    Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(websocket_handler))
}
```
