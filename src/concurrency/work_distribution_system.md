# concurrency/work_distribution_system

Reference implementation. Read after attempting the exercise.

```rust
fn work_distribution_system(work_items: Vec<i32>, num_workers: usize) -> i64 {
    assert!(num_workers > 0);
    if work_items.is_empty() {
        return 0;
    }

    let (work_tx, work_rx) = std::sync::mpsc::channel::<Option<i32>>();
    let work_rx = std::sync::Arc::new(std::sync::Mutex::new(work_rx));
    let (result_tx, result_rx) = std::sync::mpsc::channel::<i64>();

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

    for item in work_items {
        work_tx.send(Some(item)).unwrap();
    }

    for _ in 0..num_workers {
        work_tx.send(None).unwrap();
    }

    drop(result_tx); // Drop original sender

    let mut sum = 0i64;
    while let Ok(result) = result_rx.recv() {
        sum += result;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    sum
}
```
