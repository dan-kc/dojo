# WebSocket Implementation with Axum - Solution

```rust
async fn create_websocket_server() -> axum::Router {
    use axum::{Router, routing::get, response::Html};
    
    async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(handle_socket)
    }
    
    async fn handle_socket(mut socket: WebSocket) {
        while let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(text) => {
                        if text == "ping" {
                            if socket.send(Message::Text("pong".to_string())).await.is_err() {
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

## Explanation

This solution implements a WebSocket server with Axum:

1. **WebSocket Upgrade**: The `WebSocketUpgrade` extractor handles the protocol upgrade from HTTP to WebSocket.

2. **Message Loop**: The handler continuously receives messages and processes them based on type.

3. **Message Types**: Handling different WebSocket message types (Text, Binary, Close).

4. **Echo Functionality**: Text messages are echoed back with a prefix.

5. **Special Commands**: Implementing special behavior for "ping" and "close" commands.

6. **Binary Handling**: Binary messages return their size as a text response.

## Key Learning Points

- WebSocket connections are established through HTTP upgrade
- `WebSocketUpgrade::on_upgrade()` transitions from HTTP to WebSocket protocol
- Message handling requires a loop to continuously process incoming messages
- Different message types (Text, Binary, Close) need different handling
- Error handling is crucial - connection errors should gracefully close the socket
- WebSocket handlers are long-running async tasks