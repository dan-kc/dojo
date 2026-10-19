// cargo test --lib axum::routing::

//! Return an Axum 0.7 router with routes nested under /api:
//! GET /api/users -> "List of users"; POST /api/users -> 201 "User created";
//! GET /api/users/{id} -> "User: {id}"; DELETE /api/users/{id} -> "User {id} deleted".
//! IDs are strings. GET /api/search?q={query} returns "Searching for: {decoded query}";
//! a missing q is 400. Other successes are 200; / itself is 404.

fn create_api_router() -> axum::Router {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::Request;
    use axum::http::{Method, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_list_users() {
        let app = create_api_router();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"List of users");
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let app = create_api_router();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/users/42")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"User: 42");
    }

    #[tokio::test]
    async fn test_create_user() {
        let app = create_api_router();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"User created");
    }

    #[tokio::test]
    async fn test_delete_user() {
        let app = create_api_router();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/api/users/99")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"User 99 deleted");
    }

    #[tokio::test]
    async fn test_search_with_query() {
        let app = create_api_router();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/search?q=rust")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"Searching for: rust");
    }

    #[tokio::test]
    async fn test_root_not_found() {
        let app = create_api_router();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    #[tokio::test]
    async fn missing_query_and_decoding() {
        for (uri, status, body) in [
            ("/api/search", 400, None),
            (
                "/api/search?q=rust+lang",
                200,
                Some("Searching for: rust lang"),
            ),
        ] {
            let r = create_api_router()
                .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(r.status().as_u16(), status);
            if let Some(body) = body {
                assert_eq!(
                    to_bytes(r.into_body(), 4096).await.unwrap().as_ref(),
                    body.as_bytes()
                );
            }
        }
    }
}
