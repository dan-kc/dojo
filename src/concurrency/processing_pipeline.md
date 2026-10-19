# concurrency/processing_pipeline

Reference implementation. Read after attempting the exercise.

```rust
fn processing_pipeline(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }

    let (tx1, rx1) = std::sync::mpsc::sync_channel(8);
    let (tx2, rx2) = std::sync::mpsc::sync_channel(8);
    let (tx3, rx3) = std::sync::mpsc::sync_channel(8);

    let stage1 = std::thread::spawn(move || {
        for value in input {
            tx1.send(value * 2).unwrap();
        }
    });

    let stage2 = std::thread::spawn(move || {
        while let Ok(value) = rx1.recv() {
            tx2.send(value + 10).unwrap();
        }
    });

    let stage3 = std::thread::spawn(move || {
        while let Ok(value) = rx2.recv() {
            tx3.send(value / 3).unwrap();
        }
    });

    let mut results = Vec::new();
    while let Ok(value) = rx3.recv() {
        results.push(value);
    }

    stage1.join().unwrap();
    stage2.join().unwrap();
    stage3.join().unwrap();

    results
}
```
