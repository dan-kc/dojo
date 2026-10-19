// cargo test --lib concurrency::thread_pool_implementation::

//! Implement a fixed-size pool whose worker threads are reused across submissions.
//! submit accepts an owned closure and returns a channel carrying either its result or panic payload.
//! A panicking job must not kill the worker. Dropping a result receiver must not kill the worker.
//! shutdown and Drop close admission, finish all queued jobs and join every worker. worker_count is the configured size.
//! Use an unbounded job queue; no work stealing is required. The caller must not drop/shut down the pool from one of its own jobs.

pub struct ThreadPool {/* Choose private storage. */}
impl ThreadPool {
    pub fn new(size: std::num::NonZeroUsize) -> Self {
        todo!()
    }
    pub fn submit<F, T>(&self, f: F) -> std::sync::mpsc::Receiver<std::thread::Result<T>>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        todo!()
    }
    pub fn worker_count(&self) -> usize {
        todo!()
    }
    pub fn shutdown(self) {
        todo!()
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) { /* Implement shutdown. */
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::time::Duration;
    #[test]
    fn results_and_worker_reuse() {
        let p = ThreadPool::new(NonZeroUsize::new(1).unwrap());
        assert_eq!(p.worker_count(), 1);
        let a = p.submit(std::thread::current);
        let first = a
            .recv_timeout(Duration::from_secs(3))
            .unwrap()
            .unwrap()
            .id();
        let second = p
            .submit(|| std::thread::current().id())
            .recv_timeout(Duration::from_secs(3))
            .unwrap()
            .unwrap();
        assert_eq!(first, second);
        p.shutdown();
    }
    #[test]
    fn panic_does_not_kill_worker() {
        let p = ThreadPool::new(NonZeroUsize::new(1).unwrap());
        assert!(p
            .submit(|| panic!("job panic"))
            .recv_timeout(Duration::from_secs(3))
            .unwrap()
            .is_err());
        assert_eq!(
            p.submit(|| 42)
                .recv_timeout(Duration::from_secs(3))
                .unwrap()
                .unwrap(),
            42
        );
    }
    #[test]
    fn drop_drains_and_dropped_receiver_is_ok() {
        let count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        {
            let p = ThreadPool::new(NonZeroUsize::new(2).unwrap());
            for _ in 0..20 {
                let count = count.clone();
                drop(p.submit(move || {
                    count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }));
            }
        }
        assert_eq!(count.load(std::sync::atomic::Ordering::SeqCst), 20);
    }
    #[test]
    fn workers_overlap() {
        let p = ThreadPool::new(NonZeroUsize::new(2).unwrap());
        let (tx, rx) = std::sync::mpsc::channel();
        let (wait, release) = std::sync::mpsc::channel();
        let first = p.submit(move || {
            tx.send(()).unwrap();
            release.recv_timeout(Duration::from_secs(3)).unwrap();
        });
        rx.recv_timeout(Duration::from_secs(3)).unwrap();
        let second = p.submit(move || {
            wait.send(()).unwrap();
        });
        second
            .recv_timeout(Duration::from_secs(3))
            .unwrap()
            .unwrap();
        first.recv_timeout(Duration::from_secs(3)).unwrap().unwrap();
    }
}
