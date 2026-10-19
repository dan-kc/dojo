// cargo test --lib axum::basic_server::

//! Return an Axum 0.7 router. GET / returns "Hello, World!"; GET /health returns "OK";
//! GET /echo/{message} returns the decoded message path segment. All succeed with status 200.
//! Unknown routes return 404, unsupported methods 405. Constructing the router does not start a server.

fn create_basic_server() -> axum::Router {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::body::Body;
    use axum::http::Request;
    use axum::http::StatusCode;
    use tower::ServiceExt;

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
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/echo/test-message")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/nonexistent")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
