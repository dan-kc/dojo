# tokio/async_closure_usage

Reference implementation. Read after attempting the exercise.

```rust
pub async fn async_closure_usage<T, U, E, F, Fut>(
    items: Vec<T>,
    mut operation: F,
) -> Result<Vec<U>, E>
where
    F: FnMut(T) -> Fut,
    Fut: std::future::Future<Output = Result<U, E>>,
{
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        out.push(operation(item).await?);
    }
    Ok(out)
}
```
