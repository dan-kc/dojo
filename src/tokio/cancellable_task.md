# tokio/cancellable_task

Reference implementation. Read after attempting the exercise.

```rust
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
    let mut draining = false;
    loop {
        let item = if draining {
            jobs.recv().await
        } else {
            tokio::select! {biased; _=&mut shutdown=>{jobs.close();draining=true;continue;},item=jobs.recv()=>item}
        };
        let Some(item) = item else {
            return Ok(());
        };
        let result = process(item).await;
        completed
            .send(result)
            .await
            .map_err(|_| WorkerError::OutputClosed)?;
    }
}
```
