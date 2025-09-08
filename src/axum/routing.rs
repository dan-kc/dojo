// Advanced Routing with Axum
//
// Learning Objectives:
// - Master route parameters and query strings
// - Understand nested routing and route groups
// - Practice HTTP method routing
//
// cargo test --bin routing

/// Create an Axum router with the following routes:
/// - GET "/api/users" returns "List of users"
/// - GET "/api/users/{id}" returns "User: {id}"
/// - POST "/api/users" returns "User created" with status 201
/// - DELETE "/api/users/{id}" returns "User {id} deleted"
/// - GET "/api/search?q={query}" returns "Searching for: {query}"
/// 
/// All routes should be nested under "/api" prefix.
fn create_api_router() -> axum::Router {
    todo!("Implement API router with nested routes and parameters")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{StatusCode, Method};
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;
    use axum::http::Request;

    #[tokio::test]
    async fn test_list_users() {
        let app = create_api_router();
        
        let response = app
            .oneshot(Request::builder()
                .method(Method::GET)
                .uri("/api/users")
                .body(Body::empty())
                .unwrap())
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
            .oneshot(Request::builder()
                .method(Method::GET)
                .uri("/api/users/42")
                .body(Body::empty())
                .unwrap())
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
            .oneshot(Request::builder()
                .method(Method::POST)
                .uri("/api/users")
                .body(Body::empty())
                .unwrap())
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
            .oneshot(Request::builder()
                .method(Method::DELETE)
                .uri("/api/users/99")
                .body(Body::empty())
                .unwrap())
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
            .oneshot(Request::builder()
                .method(Method::GET)
                .uri("/api/search?q=rust")
                .body(Body::empty())
                .unwrap())
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
            .oneshot(Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}