// cargo test --lib tokio::cancellable_task::

//! Run a sequential background worker over a bounded job receiver and report each completed result.
//! On a shutdown signal or dropped shutdown sender, close job admission, finish the current job and drain buffered jobs.
//! Normal job-channel closure also drains and exits. Never detach processing work. If the completion receiver
//! is dropped, return OutputClosed and stop. Processing itself is infallible and must finish eventually.
//! Shutdown does not interrupt a running operation or a blocked completion send; consumers must keep draining output.

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerError {
    OutputClosed,
}
pub async fn run_worker<T, U, F, Fut>(
    mut jobs: tokio::sync::mpsc::Receiver<T>,
    mut shutdown: tokio::sync::oneshot::Receiver<()>,
    completed: tokio::sync::mpsc::Sender<U>,
    mut process: F,
) -> Result<(), WorkerError>
where
    F: FnMut(T) -> Fut,
    Fut: std::future::Future<Output = U>,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::{mpsc, oneshot};
    #[tokio::test]
    async fn closed_input_drains() {
        let (tx, rx) = mpsc::channel(3);
        tx.send(2).await.unwrap();
        tx.send(3).await.unwrap();
        drop(tx);
        let (_stop, shutdown) = oneshot::channel();
        let (out, mut results) = mpsc::channel(3);
        assert_eq!(
            run_worker(rx, shutdown, out, |n| async move { n * 2 }).await,
            Ok(())
        );
        assert_eq!(results.recv().await, Some(4));
        assert_eq!(results.recv().await, Some(6));
        assert_eq!(results.recv().await, None);
    }
    #[tokio::test]
    async fn shutdown_closes_admission_and_drains() {
        let (tx, rx) = mpsc::channel(3);
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        let (stop, shutdown) = oneshot::channel();
        stop.send(()).unwrap();
        let (out, mut results) = mpsc::channel(3);
        run_worker(rx, shutdown, out, |n| async move { n })
            .await
            .unwrap();
        assert!(tx.send(3).await.is_err());
        assert_eq!(results.recv().await, Some(1));
        assert_eq!(results.recv().await, Some(2));
        assert_eq!(results.recv().await, None);
    }
    #[tokio::test]
    async fn signal_during_work_preserves_completion() {
        let (tx, rx) = mpsc::channel(2);
        tx.send(7).await.unwrap();
        let (stop, shutdown) = oneshot::channel();
        let (started, start) = oneshot::channel();
        let (release, ready) = oneshot::channel();
        let mut gate = Some((started, ready));
        let (out, mut results) = mpsc::channel(2);
        let worker = tokio::spawn(run_worker(rx, shutdown, out, move |n| {
            let gate = gate.take();
            async move {
                if let Some((started, ready)) = gate {
                    started.send(()).unwrap();
                    ready.await.unwrap();
                }
                n
            }
        }));
        start.await.unwrap();
        stop.send(()).unwrap();
        assert!(results.try_recv().is_err());
        release.send(()).unwrap();
        assert_eq!(worker.await.unwrap(), Ok(()));
        assert_eq!(results.recv().await, Some(7));
        assert!(tx.send(8).await.is_err());
    }
    #[tokio::test]
    async fn dropped_output_is_error() {
        let (tx, rx) = mpsc::channel(1);
        tx.send(1).await.unwrap();
        drop(tx);
        let (_stop, shutdown) = oneshot::channel();
        let (out, results) = mpsc::channel::<i32>(1);
        drop(results);
        assert_eq!(
            run_worker(rx, shutdown, out, |n| async move { n }).await,
            Err(WorkerError::OutputClosed)
        );
    }
    #[tokio::test]
    async fn dropped_shutdown_sender_stops_idle_worker() {
        let (_tx, rx) = mpsc::channel::<u8>(1);
        let (stop, shutdown) = oneshot::channel();
        drop(stop);
        let (out, _results) = mpsc::channel(1);
        assert_eq!(
            run_worker(rx, shutdown, out, |n| async move { n }).await,
            Ok(())
        );
    }
}
