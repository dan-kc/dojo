// JSON API with Axum
//
// Learning Objectives:
// - Handle JSON request and response bodies
// - Work with serde for serialization/deserialization
// - Practice validation and error handling
//
// cargo test --bin json_api

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

/// Create a JSON API with the following endpoints:
/// - GET "/users" returns a list of users as JSON
/// - GET "/users/{id}" returns a single user as JSON (404 if not found)
/// - POST "/users" accepts CreateUserRequest JSON and returns created User with generated ID
/// - PUT "/users/{id}" accepts UpdateUserRequest JSON and returns updated User (404 if not found)
/// 
/// Store users in memory (can use a static Vec or similar for testing).
/// IDs should start at 1 and increment for each new user.
fn create_json_api() -> axum::Router {
    todo!("Implement JSON API with CRUD operations for users")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{StatusCode, header};
    use axum::body::{Body, to_bytes};
    use tower::ServiceExt;
    use axum::http::Request;

    #[tokio::test]
    async fn test_get_users_empty() {
        let app = create_json_api();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/users")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let users: Vec<User> = serde_json::from_slice(&body).unwrap();
        assert_eq!(users.len(), 0);
    }

    #[tokio::test]
    async fn test_create_and_get_user() {
        let app = create_json_api();
        
        // Create user
        let create_request = CreateUserRequest {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        
        let response = app.clone()
            .oneshot(Request::builder()
                .method("POST")
                .uri("/users")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&create_request).unwrap()))
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let created_user: User = serde_json::from_slice(&body).unwrap();
        assert_eq!(created_user.id, 1);
        assert_eq!(created_user.name, "Alice");
        assert_eq!(created_user.email, "alice@example.com");
        
        // Get user by ID
        let response = app
            .oneshot(Request::builder()
                .uri("/users/1")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let fetched_user: User = serde_json::from_slice(&body).unwrap();
        assert_eq!(fetched_user, created_user);
    }

    #[tokio::test]
    async fn test_update_user() {
        let app = create_json_api();
        
        // Create user first
        let create_request = CreateUserRequest {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        };
        
        app.clone()
            .oneshot(Request::builder()
                .method("POST")
                .uri("/users")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&create_request).unwrap()))
                .unwrap())
            .await
            .unwrap();
        
        // Update user
        let update_request = UpdateUserRequest {
            name: Some("Robert".to_string()),
            email: None,
        };
        
        let response = app.clone()
            .oneshot(Request::builder()
                .method("PUT")
                .uri("/users/1")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&update_request).unwrap()))
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let updated_user: User = serde_json::from_slice(&body).unwrap();
        assert_eq!(updated_user.name, "Robert");
        assert_eq!(updated_user.email, "bob@example.com");
    }

    #[tokio::test]
    async fn test_user_not_found() {
        let app = create_json_api();
        
        let response = app
            .oneshot(Request::builder()
                .uri("/users/999")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_multiple_users() {
        let app = create_json_api();
        
        // Create multiple users
        for i in 1..=3 {
            let create_request = CreateUserRequest {
                name: format!("User{}", i),
                email: format!("user{}@example.com", i),
            };
            
            app.clone()
                .oneshot(Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&create_request).unwrap()))
                    .unwrap())
                .await
                .unwrap();
        }
        
        // Get all users
        let response = app
            .oneshot(Request::builder()
                .uri("/users")
                .body(Body::empty())
                .unwrap())
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let users: Vec<User> = serde_json::from_slice(&body).unwrap();
        assert_eq!(users.len(), 3);
    }
}