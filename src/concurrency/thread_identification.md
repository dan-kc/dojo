# concurrency/thread_identification

Reference implementation. Read after attempting the exercise.

```rust
fn thread_identification(count: usize) -> Vec<(String, String)> {
    let handles: Vec<_> = (0..count)
        .map(|i| {
            std::thread::Builder::new()
                .name(format!("worker-{}", i))
                .spawn(move || {
                    let current = std::thread::current();
                    let name = current.name().unwrap_or("unnamed").to_string();
                    let id = format!("{:?}", current.id());
                    (name, id)
                })
                .expect("Failed to spawn thread")
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}
```
