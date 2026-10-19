// cargo test --lib tokio::blocking_bridge::

//! Run a caller-supplied blocking computation on Tokio's blocking pool and await its result.
//! Return its value, preserving JoinError on panic. The async runtime must remain able to run other tasks.
//! Inputs and outputs are owned and Send. This exercise does not promise cancellation of an already-running blocking job.

pub async fn blocking_bridge<F, T>(operation: F) -> Result<T, tokio::task::JoinError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn owned_result() {
        assert_eq!(
            blocking_bridge(|| String::from("result")).await.unwrap(),
            "result"
        );
    }
    #[tokio::test]
    async fn panic_is_join_error() {
        assert!(blocking_bridge(|| panic!("failure"))
            .await
            .unwrap_err()
            .is_panic());
    }
    #[tokio::test]
    async fn does_not_block_current_thread_runtime() {
        let (tx, rx) = std::sync::mpsc::channel();
        let (started, start) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(blocking_bridge(move || {
            started.send(()).unwrap();
            rx.recv_timeout(std::time::Duration::from_secs(3)).unwrap()
        }));
        start.await.unwrap();
        tx.send(42).unwrap();
        assert_eq!(task.await.unwrap().unwrap(), 42);
    }
}
