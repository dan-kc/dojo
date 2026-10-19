# axum/state_management

Reference implementation. Read after attempting the exercise.

```rust
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
struct AppState {
    request_count: Arc<AtomicU64>,
    visitors: Arc<Mutex<HashMap<String, u32>>>,
}
fn create_stateful_router() -> axum::Router {
    use axum::{
        extract::{Path, State},
        routing::{get, post},
        Json, Router,
    };
    use serde_json::json;

    let state = AppState {
        request_count: Arc::new(AtomicU64::new(0)),
        visitors: Arc::new(Mutex::new(HashMap::new())),
    };

    async fn handle_root(State(state): State<AppState>) -> String {
        let count = state.request_count.fetch_add(1, Ordering::SeqCst) + 1;
        format!("Request #{}", count)
    }

    async fn handle_stats(State(state): State<AppState>) -> Json<serde_json::Value> {
        state.request_count.fetch_add(1, Ordering::SeqCst);

        let total_requests = state.request_count.load(Ordering::SeqCst);
        let visitors = state.visitors.lock().unwrap();
        let unique_visitors = visitors.len();

        Json(json!({
            "total_requests": total_requests,
            "unique_visitors": unique_visitors
        }))
    }

    async fn handle_visitor(Path(ip): Path<String>, State(state): State<AppState>) -> String {
        state.request_count.fetch_add(1, Ordering::SeqCst);

        let mut visitors = state.visitors.lock().unwrap();
        let visit_count = visitors.entry(ip).and_modify(|c| *c += 1).or_insert(1);

        format!("Visit count: {}", *visit_count)
    }

    Router::new()
        .route("/", get(handle_root))
        .route("/stats", get(handle_stats))
        .route("/visitor/:ip", post(handle_visitor))
        .with_state(state)
}
```
