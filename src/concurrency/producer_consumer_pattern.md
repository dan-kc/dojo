**Solution:**

```rust
fn producer_consumer_pattern(producers: Vec<(i32, i32)>) -> Vec<i32> {
    if producers.is_empty() {
        return Vec::new();
    }
    
    let (tx, rx) = std::sync::mpsc::channel();
    
    let handles: Vec<_> = producers
        .into_iter()
        .map(|(start, end)| {
            let tx = tx.clone();
            std::thread::spawn(move || {
                for i in start..=end {
                    tx.send(i).unwrap();
                }
            })
        })
        .collect();
    
    drop(tx); // Drop original sender so receiver knows when all producers are done
    
    let mut results = Vec::new();
    while let Ok(value) = rx.recv() {
        results.push(value);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    results
}
```

**Explanation:**

This solution demonstrates the classic producer-consumer pattern using Rust's multi-producer, single-consumer (mpsc) channels. Key concepts:

1. **Channel Creation**: `mpsc::channel()` creates an unbounded channel with a sender (tx) and receiver (rx)
2. **Sender Cloning**: Multiple producers can send to the same channel by cloning the sender
3. **Channel Closing**: Dropping the original sender after cloning ensures the receiver knows when all producers are done
4. **Blocking Receive**: `recv()` blocks until a message is available or all senders are dropped

This pattern is essential for work distribution where multiple sources produce data consumed by a single processor. The channel acts as a thread-safe queue, handling synchronization automatically.

OR

```rust
fn producer_consumer_pattern(producers: Vec<(i32, i32)>) -> Vec<i32> {
    let (tx, rx) = std::sync::mpmc::channel();

    let mut total = 0;
    for p in producers {
        let tx = tx.clone();
        total += p.1 - p.0 + 1;
        std::thread::spawn(move || {
            for num in p.0..=p.1 {
                tx.send(num).unwrap();
            }
        });
    }

    drop(tx);

    let mut res = Vec::with_capacity(total as usize);

    res.extend(rx);
    res.sort();
    res
}
```
This solution avoids the handles because we don't actually need the threads to finish, we just need
all txs to drop. Also this solution pre-allocates for the final vec.
