**Solution:**

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

**Explanation:**

This solution demonstrates thread metadata management in Rust. Key concepts:

1. **Thread Builder**: Using `thread::Builder` allows setting thread properties like name before spawning
2. **Thread Current**: `thread::current()` provides access to the currently executing thread's metadata
3. **Thread ID**: Each thread has a unique ID accessible via `thread::id()`, formatted using Debug trait
4. **Named Threads**: Named threads are easier to debug and monitor in production systems

Thread naming is particularly useful for debugging multi-threaded applications, as thread names appear in debugger output and panic messages. The thread ID is guaranteed to be unique for the lifetime of the thread.