# axum/error_handling

Reference implementation. Read after attempting the exercise.

```rust
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
enum ApiError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

use axum::Json;
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, "NotFound", msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BadRequest", msg),
            ApiError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "InternalError", msg)
            }
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "authentication required".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message,
        });

        (status, body).into_response()
    }
}

fn create_error_handling_router() -> axum::Router {
    use axum::{extract::Path, routing::get, Router};

    async fn get_resource(Path(id): Path<String>) -> Result<String, ApiError> {
        if id.parse::<u32>().is_ok() {
            Ok(format!("Resource: {}", id))
        } else {
            Err(ApiError::BadRequest("Invalid resource ID".to_string()))
        }
    }

    async fn protected_route() -> Result<String, ApiError> {
        Err(ApiError::Unauthorized)
    }

    async fn failing_route() -> Result<String, ApiError> {
        Err(ApiError::InternalError(
            "An internal error occurred".to_string(),
        ))
    }

    async fn missing_route() -> Result<String, ApiError> {
        Err(ApiError::NotFound("Resource not found".to_string()))
    }

    Router::new()
        .route("/resource/:id", get(get_resource))
        .route("/protected", get(protected_route))
        .route("/fail", get(failing_route))
        .route("/missing", get(missing_route))
}
```
