**Solution:**

```rust
fn parallel_counter(num_threads: usize, sleep_ms: u64) -> u32 {
    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            std::thread::spawn(move || {
                let _ = std::time::Duration::from_millis(sleep_ms);
                1u32
            })
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .fold(0, |acc, e| acc + e)
}
```

**Explanation:**

This solution demonstrates basic thread spawning and synchronization in Rust. The function creates `num_threads` threads, each of which sleeps for the specified duration and returns 1. The key concepts:

1. **Thread Creation**: Using `thread::spawn` creates new OS threads that run concurrently
2. **JoinHandles**: Each spawned thread returns a `JoinHandle` that allows us to wait for completion and retrieve the result
3. **Parallel Execution**: All threads run simultaneously, so the total time is approximately equal to the sleep duration (not multiplied by thread count)
4. **Result Collection**: The `join()` method blocks until the thread completes and returns its result, which we sum up

The use of `move` in the closure is important to transfer ownership of the sleep duration into the thread's closure.
