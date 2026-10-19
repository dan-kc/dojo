# tokio/spawn_and_sum_tasks

Reference implementation. Read after attempting the exercise.

```rust
pub async fn spawn_and_sum_tasks<F>(jobs: Vec<F>) -> Result<i64, tokio::task::JoinError>
where
    F: std::future::Future<Output = i64> + Send + 'static,
{
    let mut tasks = tokio::task::JoinSet::new();
    for job in jobs {
        tasks.spawn(job);
    }
    let mut sum = 0;
    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(value) => sum += value,
            Err(error) => {
                tasks.shutdown().await;
                return Err(error);
            }
        }
    }
    Ok(sum)
}
```
