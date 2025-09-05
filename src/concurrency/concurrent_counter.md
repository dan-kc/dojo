**Solution:**

```rust
fn concurrent_counter(num_threads: usize, increments_per_thread: usize) -> usize {
    let counter = std::sync::Arc::new(std::sync::Mutex::new(0usize));
    let mut handles = Vec::new();
    
    for _ in 0..num_threads {
        let counter = counter.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut count = counter.lock().unwrap();
                *count += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let final_count = *counter.lock().unwrap();
    final_count
}
```

**Explanation:**

This solution demonstrates the fundamental Arc<Mutex<T>> pattern for shared mutable state. Key concepts:

1. **Arc (Atomic Reference Counting)**: Enables multiple threads to share ownership of the same data
2. **Mutex (Mutual Exclusion)**: Ensures only one thread can access the data at a time
3. **Lock Guard**: The `lock()` method returns a guard that automatically releases the lock when dropped
4. **Clone for Sharing**: Arc::clone creates a new reference to the same data, incrementing the reference count

This pattern is essential for sharing mutable state between threads safely. The Mutex prevents data races, while Arc allows multiple threads to own references to the Mutex. The combination provides both thread safety and shared ownership.