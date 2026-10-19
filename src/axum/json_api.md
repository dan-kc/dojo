# axum/json_api

Reference implementation. Read after attempting the exercise.

```rust
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
fn create_json_api() -> axum::Router {
    use axum::{
        extract::{Path, State},
        http::StatusCode,
        routing::{get, post, put},
        Json, Router,
    };
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

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
        let mut users: Vec<User> = state.users.values().cloned().collect();
        users.sort_by_key(|u| u.id);
        Json(users)
    }

    async fn get_user(
        Path(id): Path<u32>,
        State(state): State<SharedState>,
    ) -> Result<Json<User>, StatusCode> {
        let state = state.lock().unwrap();
        state
            .users
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
