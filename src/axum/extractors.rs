// cargo test --lib axum::extractors::

//! Implement custom request-parts extractors ApiKey and ValidatedPagination for Axum 0.7.
//! GET /protected requires X-API-Key and returns "Access granted: {key}". Missing -> 401,
//! non-text header -> 400, key lacking the case-sensitive valid- prefix -> 403.
//! GET /items returns PaginatedResponse over "Item 1" through "Item 1000". Defaults page=1,
//! limit=10; clamp limit to 100; reject zero or malformed unsigned parameters with 400.
//! Use 1-based pages, an empty list beyond the end, and total=1000. Large pages must not overflow.

#[derive(Debug, Clone)]
struct ApiKey(String);
#[derive(Debug, serde::Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}
#[derive(Debug, serde::Serialize)]
struct PaginatedResponse<T> {
    data: Vec<T>,
    page: u32,
    limit: u32,
    total: u32,
}
struct ValidatedPagination {
    page: u32,
    limit: u32,
}
fn create_extractor_router() -> axum::Router {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::StatusCode;
    use axum::http::{header, Request};
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_api_key_valid() {
        let app = create_extractor_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("X-API-Key", "valid-secret123")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_api_key_invalid() {
        let app = create_extractor_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header("X-API-Key", "invalid-key")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_pagination_defaults() {
        let app = create_extractor_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/items")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/items?page=2&limit=5")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/items?limit=200")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/items")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let paginated: Value = serde_json::from_slice(&body).unwrap();

        // Should have a total count
        assert!(paginated["total"].as_u64().unwrap() > 0);
    }
    #[tokio::test]
    async fn boundaries() {
        for uri in ["/items?page=0", "/items?limit=0", "/items?page=no"] {
            let r = create_extractor_router()
                .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(r.status(), StatusCode::BAD_REQUEST);
        }
        let r = create_extractor_router()
            .oneshot(
                Request::builder()
                    .uri("/items?page=4294967295&limit=100")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(r.status(), StatusCode::OK);
        let v: Value =
            serde_json::from_slice(&to_bytes(r.into_body(), 4096).await.unwrap()).unwrap();
        assert_eq!(v["data"], serde_json::json!([]));
        assert_eq!(v["total"], 1000);
    }
}
