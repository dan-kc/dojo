# tokio/blocking_bridge

Reference implementation. Read after attempting the exercise.

```rust
pub async fn blocking_bridge<F, T>(operation: F) -> Result<T, tokio::task::JoinError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(operation).await
}
```
