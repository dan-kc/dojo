**Solution:**

```rust
fn broadcast_system(message: String, num_receivers: usize) -> Vec<String> {
    if num_receivers == 0 {
        return Vec::new();
    }
    
    // Create channels for each receiver
    let mut senders = Vec::new();
    let mut receivers = Vec::new();
    
    for _ in 0..num_receivers {
        let (tx, rx) = std::sync::mpsc::channel();
        senders.push(tx);
        receivers.push(rx);
    }
    
    // Relay thread that broadcasts to all receivers
    let relay_thread = std::thread::spawn(move || {
        for tx in senders {
            tx.send(message.clone()).unwrap();
        }
    });
    
    // Collect from all receivers
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

**Explanation:**

This solution demonstrates how to implement broadcasting with Rust's single-consumer channels. Key concepts:

1. **Channel Per Receiver**: Since mpsc channels have only one consumer, we create a separate channel for each receiver
2. **Relay Thread**: A dedicated thread distributes the message to all channel senders
3. **Message Cloning**: The message is cloned for each receiver to maintain ownership rules
4. **Synchronization**: The relay thread ensures all messages are sent before the function returns

This pattern is useful when you need to notify multiple components of an event. For production use, consider using broadcast channels from external crates like `tokio::sync::broadcast` which provide native multi-consumer support.