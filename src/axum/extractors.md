# Custom Extractors with Axum - Solution

```rust
#[async_trait]
impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);
    
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header_value = parts
            .headers
            .get("x-api-key")
            .ok_or((StatusCode::UNAUTHORIZED, "Missing API key"))?;
        
        let key = header_value
            .to_str()
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid API key format"))?;
        
        if !key.starts_with("valid-") {
            return Err((StatusCode::FORBIDDEN, "Invalid API key"));
        }
        
        Ok(ApiKey(key.to_string()))
    }
}

struct ValidatedPagination {
    page: u32,
    limit: u32,
}

#[async_trait]
impl<S> FromRequestParts<S> for ValidatedPagination
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);
    
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(params): Query<Pagination> = Query::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid query parameters"))?;
        
        let page = params.page.unwrap_or(1);
        let limit = params.limit.unwrap_or(10).min(100);
        
        Ok(ValidatedPagination { page, limit })
    }
}

async fn create_extractor_router() -> axum::Router {
    use axum::{Router, routing::get};
    
    async fn protected_handler(ApiKey(key): ApiKey) -> String {
        format!("Access granted: {}", key)
    }
    
    async fn items_handler(pagination: ValidatedPagination) -> Json<PaginatedResponse<String>> {
        let total = 1000;
        let start = ((pagination.page - 1) * pagination.limit) as usize;
        let end = (start + pagination.limit as usize).min(total as usize);
        
        let data: Vec<String> = (start..end)
            .map(|i| format!("Item {}", i + 1))
            .collect();
        
        Json(PaginatedResponse {
            data,
            page: pagination.page,
            limit: pagination.limit,
            total,
        })
    }
    
    Router::new()
        .route("/protected", get(protected_handler))
        .route("/items", get(items_handler))
}
```

## Explanation

This solution demonstrates custom extractor implementation:

1. **FromRequestParts Trait**: Implementing this trait allows types to be extracted from requests.

2. **Header Extraction**: Reading custom headers from the request parts.

3. **Validation Logic**: Performing validation within the extractor, returning appropriate errors.

4. **Query Parameter Processing**: Extracting and validating query parameters with defaults.

5. **Error Responses**: Returning status codes and messages for extraction failures.

## Key Learning Points

- Custom extractors implement `FromRequestParts` for request data extraction
- Extractors can access headers, query params, and other request parts
- Validation logic in extractors keeps handlers clean
- Rejection types determine error responses when extraction fails
- Extractors can compose other extractors (like using Query within custom extractor)
- Default values and limits can be enforced within extractors