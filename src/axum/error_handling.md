# Error Handling with Axum - Solution

```rust
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NotFound",
                msg
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BadRequest",
                msg
            ),
            ApiError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "InternalError",
                msg
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Authentication required".to_string()
            ),
        };
        
        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message,
        });
        
        (status, body).into_response()
    }
}

async fn create_error_handling_router() -> axum::Router {
    use axum::{Router, routing::get, extract::Path};
    
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
        Err(ApiError::InternalError("An internal error occurred".to_string()))
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

## Explanation

This solution implements comprehensive error handling:

1. **IntoResponse Trait**: Custom implementation converts errors into HTTP responses with appropriate status codes.

2. **Error Variants**: Different error types map to different HTTP status codes (404, 400, 500, 401).

3. **JSON Error Responses**: All errors return consistent JSON structure with error type and message.

4. **Result Return Types**: Handlers return `Result<T, ApiError>` for automatic error conversion.

5. **Error Context**: Each error variant can carry additional context in its message.

## Key Learning Points

- `IntoResponse` trait enables custom types to be converted to HTTP responses
- Error types should map to appropriate HTTP status codes
- Consistent error response structure improves API usability
- Result types in handlers enable clean error propagation
- Pattern matching on error variants allows flexible error handling
- JSON serialization of errors provides machine-readable error responses