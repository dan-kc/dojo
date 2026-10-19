// cargo test --lib axum::json_api::

//! Build an in-memory JSON user API. Each newly constructed router starts empty; clones share
//! state. GET /users returns users ordered by ID. GET /users/{id} returns a user or 404.
//! POST /users accepts CreateUserRequest, allocates consecutive IDs from 1 atomically and returns
//! 201 with the User. PUT /users/{id} applies only Some fields and returns the updated user or 404.
//! Use the request/response types below. Concurrent requests must not lose updates or duplicate IDs.
//! No lock may be held across await. ID exhaustion is outside the exercise input range.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    email: Option<String>,
}

fn create_json_api() -> axum::Router {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::{to_bytes, Body};
    use axum::http::Request;
    use axum::http::{header, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_users_empty() {
        let app = create_json_api();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
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

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&create_request).unwrap()))
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/users/1")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&create_request).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Update user
        let update_request = UpdateUserRequest {
            name: Some("Robert".to_string()),
            email: None,
        };

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/users/1")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(serde_json::to_string(&update_request).unwrap()))
                    .unwrap(),
            )
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
            .oneshot(
                Request::builder()
                    .uri("/users/999")
                    .body(Body::empty())
                    .unwrap(),
            )
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
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/users")
                        .header(header::CONTENT_TYPE, "application/json")
                        .body(Body::from(serde_json::to_string(&create_request).unwrap()))
                        .unwrap(),
                )
                .await
                .unwrap();
        }

        // Get all users
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let users: Vec<User> = serde_json::from_slice(&body).unwrap();
        assert_eq!(users.len(), 3);
    }
    #[tokio::test]
    async fn independent_routers_and_concurrent_ids() {
        let app = create_json_api();
        let mut jobs = tokio::task::JoinSet::new();
        for _ in 0..20 {
            let app = app.clone();
            jobs.spawn(async move {
                let r = app
                    .oneshot(
                        Request::builder()
                            .method("POST")
                            .uri("/users")
                            .header("content-type", "application/json")
                            .body(Body::from(r#"{"name":"n","email":"e"}"#))
                            .unwrap(),
                    )
                    .await
                    .unwrap();
                assert_eq!(r.status(), StatusCode::CREATED);
                let u: User =
                    serde_json::from_slice(&to_bytes(r.into_body(), 4096).await.unwrap()).unwrap();
                u.id
            });
        }
        let mut ids = Vec::new();
        while let Some(id) = jobs.join_next().await {
            ids.push(id.unwrap());
        }
        ids.sort();
        assert_eq!(ids, (1..=20).collect::<Vec<_>>());
        let r = create_json_api()
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(to_bytes(r.into_body(), 4096).await.unwrap().as_ref(), b"[]");
        let r = app
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let users: Vec<User> =
            serde_json::from_slice(&to_bytes(r.into_body(), 8192).await.unwrap()).unwrap();
        assert_eq!(users.iter().map(|u| u.id).collect::<Vec<_>>(), ids);
    }
}
