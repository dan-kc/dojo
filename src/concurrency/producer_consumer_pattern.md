# concurrency/producer_consumer_pattern

Reference implementation. Read after attempting the exercise.

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

    results.sort_unstable();
    results
}
```
