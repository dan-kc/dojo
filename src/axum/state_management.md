# Shared State Management with Axum - Solution

```rust
async fn create_stateful_router() -> axum::Router {
    use axum::{
        Router,
        routing::{get, post},
        extract::{State, Path},
        Json,
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
    
    async fn handle_visitor(
        Path(ip): Path<String>,
        State(state): State<AppState>,
    ) -> String {
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

## Explanation

This solution demonstrates concurrent state management in Axum:

1. **Atomic Operations**: Using `AtomicU64` for lock-free concurrent counter updates.

2. **Thread-Safe Collections**: Wrapping HashMap in `Arc<Mutex<>>` for safe concurrent access.

3. **Memory Ordering**: Using `Ordering::SeqCst` for sequential consistency in atomic operations.

4. **Entry API**: Using HashMap's `entry()` API for efficient insert-or-update operations.

5. **State Cloning**: The `AppState` derives `Clone` to be passed to multiple handlers.

## Key Learning Points

- Atomic types provide lock-free concurrent access for simple values
- `Arc` enables sharing ownership across async tasks
- `Mutex` provides exclusive access to shared mutable data
- The `fetch_add` operation atomically increments and returns the old value
- State must be `Clone + Send + Sync` to work with Axum's async handlers
- Different synchronization primitives suit different use cases (atomics vs mutexes)