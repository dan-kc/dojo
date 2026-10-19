# concurrency/broadcast_system

Reference implementation. Read after attempting the exercise.

```rust
fn broadcast_system(message: String, num_receivers: usize) -> Vec<String> {
    if num_receivers == 0 {
        return Vec::new();
    }

    let mut senders = Vec::new();
    let mut receivers = Vec::new();

    for _ in 0..num_receivers {
        let (tx, rx) = std::sync::mpsc::channel();
        senders.push(tx);
        receivers.push(rx);
    }

    let relay_thread = std::thread::spawn(move || {
        for tx in senders {
            tx.send(message.clone()).unwrap();
        }
    });

    let mut results = Vec::new();
    for rx in receivers {
        if let Ok(msg) = rx.recv() {
            results.push(msg);
        }
    }

    relay_thread.join().unwrap();

    results
}
```
