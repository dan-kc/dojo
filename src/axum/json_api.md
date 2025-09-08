# JSON API with Axum - Solution

```rust
async fn create_json_api() -> axum::Router {
    use axum::{
        Router,
        routing::{get, post, put},
        extract::{Path, State},
        Json,
        http::StatusCode,
    };
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    
    type SharedState = Arc<Mutex<AppState>>;
    
    struct AppState {
        users: HashMap<u32, User>,
        next_id: u32,
    }
    
    let state = Arc::new(Mutex::new(AppState {
        users: HashMap::new(),
        next_id: 1,
    }));
    
    async fn get_users(State(state): State<SharedState>) -> Json<Vec<User>> {
        let state = state.lock().unwrap();
        let users: Vec<User> = state.users.values().cloned().collect();
        Json(users)
    }
    
    async fn get_user(
        Path(id): Path<u32>,
        State(state): State<SharedState>,
    ) -> Result<Json<User>, StatusCode> {
        let state = state.lock().unwrap();
        state.users
            .get(&id)
            .cloned()
            .map(Json)
            .ok_or(StatusCode::NOT_FOUND)
    }
    
    async fn create_user(
        State(state): State<SharedState>,
        Json(request): Json<CreateUserRequest>,
    ) -> (StatusCode, Json<User>) {
        let mut state = state.lock().unwrap();
        let user = User {
            id: state.next_id,
            name: request.name,
            email: request.email,
        };
        state.users.insert(user.id, user.clone());
        state.next_id += 1;
        (StatusCode::CREATED, Json(user))
    }
    
    async fn update_user(
        Path(id): Path<u32>,
        State(state): State<SharedState>,
        Json(request): Json<UpdateUserRequest>,
    ) -> Result<Json<User>, StatusCode> {
        let mut state = state.lock().unwrap();
        let user = state.users.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
        
        if let Some(name) = request.name {
            user.name = name;
        }
        if let Some(email) = request.email {
            user.email = email;
        }
        
        Ok(Json(user.clone()))
    }
    
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user))
        .with_state(state)
}
```

## Explanation

This solution implements a complete JSON CRUD API:

1. **Shared State**: Using `Arc<Mutex<>>` to share mutable state across async handlers safely.

2. **JSON Serialization**: The `Json` extractor automatically deserializes request bodies and serializes responses.

3. **Error Handling**: Returning `Result` types allows handlers to return different status codes.

4. **State Management**: Using a HashMap to store users in memory with auto-incrementing IDs.

5. **Request/Response Types**: Separate types for create and update requests provide clear API contracts.

## Key Learning Points

- `Json<T>` extractor handles automatic serialization/deserialization
- State is shared using `with_state()` and extracted with `State<T>`
- Handlers can return `Result` to handle success and error cases differently
- Multiple HTTP methods can be chained on the same route
- Serde derives enable automatic JSON conversion for custom types