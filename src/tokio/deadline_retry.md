# tokio/deadline_retry

Reference implementation. Read after attempting the exercise.

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum RetryError<E> {
    Exhausted(E),
    TimedOut,
}
pub async fn deadline_retry<T, E, F, Fut>(
    mut operation: F,
    max_attempts: std::num::NonZeroUsize,
    budget: std::time::Duration,
    backoff: std::time::Duration,
) -> Result<T, RetryError<E>>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    if budget.is_zero() {
        return Err(RetryError::TimedOut);
    }
    let deadline = tokio::time::Instant::now() + budget;
    for attempt in 0..max_attempts.get() {
        if tokio::time::Instant::now() >= deadline {
            return Err(RetryError::TimedOut);
        }
        match tokio::time::timeout_at(deadline, operation()).await {
            Ok(Ok(value)) => return Ok(value),
            Ok(Err(e)) => {
                if attempt + 1 == max_attempts.get() {
                    return Err(RetryError::Exhausted(e));
                }
            }
            Err(_) => return Err(RetryError::TimedOut),
        }
        if tokio::time::timeout_at(deadline, tokio::time::sleep(backoff))
            .await
            .is_err()
        {
            return Err(RetryError::TimedOut);
        }
    }
    unreachable!()
}
```
