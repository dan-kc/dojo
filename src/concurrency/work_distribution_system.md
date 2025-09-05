**Solution:**

```rust
fn work_distribution_system(work_items: Vec<i32>, num_workers: usize) -> i64 {
    if work_items.is_empty() || num_workers == 0 {
        return 0;
    }
    
    let (work_tx, work_rx) = std::sync::mpsc::channel::<Option<i32>>();
    let work_rx = std::sync::Arc::new(std::sync::Mutex::new(work_rx));
    let (result_tx, result_rx) = std::sync::mpsc::channel::<i64>();
    
    // Spawn worker threads
    let handles: Vec<_> = (0..num_workers)
        .map(|_| {
            let work_rx = work_rx.clone();
            let result_tx = result_tx.clone();
            std::thread::spawn(move || {
                loop {
                    let item = {
                        let rx = work_rx.lock().unwrap();
                        rx.recv().unwrap()
                    };
                    
                    match item {
                        Some(value) => {
                            let squared = (value as i64) * (value as i64);
                            result_tx.send(squared).unwrap();
                        }
                        None => break, // Poison pill to stop worker
                    }
                }
            })
        })
        .collect();
    
    // Send work items
    for item in work_items {
        work_tx.send(Some(item)).unwrap();
    }
    
    // Send poison pills to stop workers
    for _ in 0..num_workers {
        work_tx.send(None).unwrap();
    }
    
    drop(result_tx); // Drop original sender
    
    // Collect results
    let mut sum = 0i64;
    while let Ok(result) = result_rx.recv() {
        sum += result;
    }
    
    // Wait for workers to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    sum
}
```

**Explanation:**

This solution demonstrates a work queue pattern with bidirectional communication. Key concepts:

1. **Work Queue**: Uses Arc<Mutex<Receiver>> to share the work receiver among workers
2. **Poison Pill Pattern**: Sending `None` signals workers to terminate gracefully
3. **Result Collection**: Workers send results back through a separate channel
4. **Load Balancing**: Workers pull work as they become available, naturally balancing the load

This pattern is useful for CPU-bound tasks where work items can be processed independently. The mutex around the receiver ensures only one worker gets each work item, providing automatic work distribution.