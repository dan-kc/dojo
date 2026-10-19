// cargo test --lib tokio::deadline_retry::

//! Retry a fallible asynchronous operation up to max_attempts within one overall timeout budget.
//! Return immediately on success. After an error, wait backoff before the next attempt if any remain.
//! The deadline covers attempts and backoff together; do not restart the budget. Return Exhausted(last_error)
//! after the last failed attempt, or TimedOut when the budget expires. Zero budget starts no attempt.
//! The operation must be safe to cancel by dropping its future; no background work may be detached.

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
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::time::Duration;
    #[tokio::test(start_paused = true)]
    async fn succeeds_after_retry() {
        let mut calls = 0;
        let out = deadline_retry(
            || {
                calls += 1;
                let n = calls;
                async move {
                    if n < 3 {
                        Err("try")
                    } else {
                        Ok(9)
                    }
                }
            },
            NonZeroUsize::new(4).unwrap(),
            Duration::from_secs(10),
            Duration::from_secs(1),
        )
        .await;
        assert_eq!(out, Ok(9));
        assert_eq!(calls, 3);
    }
    #[tokio::test(start_paused = true)]
    async fn budget_includes_backoff() {
        let mut calls = 0;
        let out = deadline_retry(
            || {
                calls += 1;
                async { Err::<(), _>("no") }
            },
            NonZeroUsize::new(10).unwrap(),
            Duration::from_secs(3),
            Duration::from_secs(2),
        )
        .await;
        assert_eq!(out, Err(RetryError::TimedOut));
        assert_eq!(calls, 2);
    }
    #[tokio::test(start_paused = true)]
    async fn exhaustion_and_zero_budget() {
        let out = deadline_retry(
            || async { Err::<(), _>(7) },
            NonZeroUsize::new(1).unwrap(),
            Duration::from_secs(1),
            Duration::ZERO,
        )
        .await;
        assert_eq!(out, Err(RetryError::Exhausted(7)));
        let out = deadline_retry(
            || async {
                panic!("must not run");
                #[allow(unreachable_code)]
                Ok::<(), ()>(())
            },
            NonZeroUsize::new(1).unwrap(),
            Duration::ZERO,
            Duration::ZERO,
        )
        .await;
        assert_eq!(out, Err(RetryError::TimedOut));
    }
    #[tokio::test(start_paused = true)]
    async fn cancels_pending_attempt() {
        let out = deadline_retry(
            || std::future::pending::<Result<(), ()>>(),
            NonZeroUsize::new(2).unwrap(),
            Duration::from_secs(1),
            Duration::ZERO,
        )
        .await;
        assert_eq!(out, Err(RetryError::TimedOut));
    }
}
