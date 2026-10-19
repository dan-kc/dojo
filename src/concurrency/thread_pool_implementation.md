# concurrency/thread_pool_implementation

Reference implementation. Read after attempting the exercise.

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    sender: Option<std::sync::mpsc::Sender<Job>>,
    workers: Vec<std::thread::JoinHandle<()>>,
}
impl ThreadPool {
    pub fn new(size: std::num::NonZeroUsize) -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<Job>();
        let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));
        let workers = (0..size.get())
            .map(|_| {
                let rx = rx.clone();
                std::thread::spawn(move || loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => job(),
                        Err(_) => break,
                    }
                })
            })
            .collect();
        Self {
            sender: Some(tx),
            workers,
        }
    }
    pub fn submit<F, T>(&self, f: F) -> std::sync::mpsc::Receiver<std::thread::Result<T>>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        self.sender
            .as_ref()
            .unwrap()
            .send(Box::new(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
                let _ = tx.send(result);
            }))
            .unwrap();
        rx
    }
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
    pub fn shutdown(self) {
        drop(self);
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();
        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}
```
