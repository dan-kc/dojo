# tokio/bounded_map

Reference implementation. Read after attempting the exercise.

```rust
pub async fn bounded_map<T, U, F, Fut>(
    items: Vec<T>,
    limit: std::num::NonZeroUsize,
    operation: F,
) -> Vec<U>
where
    F: FnMut(T) -> Fut,
    Fut: std::future::Future<Output = U>,
{
    use futures::{stream, StreamExt};
    stream::iter(items)
        .map(operation)
        .buffered(limit.get())
        .collect()
        .await
}
```
