// cargo test --lib tokio::spawn_and_sum_tasks::

//! Spawn every supplied asynchronous computation and sum its i64 result after completion.
//! Return the first observed task JoinError if a task panics or is cancelled, aborting and awaiting all remaining tasks
//! before returning the error. Empty input sums to zero. Assume the sum fits i64. Do not detach work.

pub async fn spawn_and_sum_tasks<F>(jobs: Vec<F>) -> Result<i64, tokio::task::JoinError>
where
    F: std::future::Future<Output = i64> + Send + 'static,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn computes_exact_sum() {
        assert_eq!(
            spawn_and_sum_tasks((1..=5).map(|n| async move { n * n }).collect())
                .await
                .unwrap(),
            55
        );
    }
    #[tokio::test(start_paused = true)]
    async fn concurrent_timers() {
        let start = tokio::time::Instant::now();
        let jobs = (0..4)
            .map(|_| async {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                2
            })
            .collect();
        assert_eq!(spawn_and_sum_tasks(jobs).await.unwrap(), 8);
        assert_eq!(start.elapsed(), std::time::Duration::from_secs(1));
    }
    #[tokio::test]
    async fn panic_cancels_remaining_work() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };
        struct Guard(Arc<AtomicUsize>);
        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
        }
        let dropped = Arc::new(AtomicUsize::new(0));
        let jobs: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = i64> + Send>>> = vec![
            {
                let g = Guard(dropped.clone());
                Box::pin(async move {
                    let _g = g;
                    std::future::pending::<i64>().await
                })
            },
            Box::pin(async { panic!("failure") }),
        ];
        assert!(spawn_and_sum_tasks(jobs).await.unwrap_err().is_panic());
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    }
    #[tokio::test]
    async fn empty() {
        let jobs: Vec<std::future::Ready<i64>> = vec![];
        assert_eq!(spawn_and_sum_tasks(jobs).await.unwrap(), 0);
    }
}
