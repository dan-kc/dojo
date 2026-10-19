// cargo test --lib axum::websockets::

//! GET / returns "WebSocket server ready". Upgrade /ws and handle each connection independently.
//! Text "ping" -> text "pong"; text "close" -> close frame and exit; other text -> "Echo: {text}".
//! Binary messages -> text "N bytes". Respond correctly to protocol Ping frames (distinct from the
//! text command). Stop on disconnect, close or I/O error. Tests use a real ephemeral local server.

fn create_websocket_server() -> axum::Router {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{SinkExt, StreamExt};
    #[tokio::test]
    async fn real_upgrade_and_frames() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, create_websocket_server())
                .await
                .unwrap()
        });
        let result = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            use tokio_tungstenite::tungstenite::Message as M;
            let (mut ws, response) = tokio_tungstenite::connect_async(format!("ws://{address}/ws"))
                .await
                .unwrap();
            assert_eq!(response.status().as_u16(), 101);
            for (sent, expected) in [
                (M::Text("雪".into()), M::Text("Echo: 雪".into())),
                (M::Text("ping".into()), M::Text("pong".into())),
                (M::Binary(vec![0, 255, 42]), M::Text("3 bytes".into())),
                (M::Ping(vec![7]), M::Pong(vec![7])),
            ] {
                ws.send(sent).await.unwrap();
                assert_eq!(ws.next().await.unwrap().unwrap(), expected);
            }
            ws.send(M::Text("close".into())).await.unwrap();
            assert!(matches!(ws.next().await.unwrap().unwrap(), M::Close(_)));
        })
        .await;
        server.abort();
        let _ = server.await;
        result.unwrap();
    }
    #[tokio::test]
    async fn http_and_no_upgrade() {
        use axum::{
            body::{to_bytes, Body},
            http::Request,
        };
        use tower::ServiceExt;
        let r = create_websocket_server()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(r.status().as_u16(), 200);
        assert_eq!(
            to_bytes(r.into_body(), 4096).await.unwrap().as_ref(),
            b"WebSocket server ready"
        );
        let r = create_websocket_server()
            .oneshot(Request::builder().uri("/ws").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert!(r.status().is_client_error());
    }
}
