**Solution:**

```rust
fn timed_message_collector(
    messages: Vec<String>,
    send_interval_ms: u64,
    collect_duration_ms: u64,
) -> Vec<String> {
    let (tx, rx) = std::sync::mpsc::channel();
    
    // Spawn sender thread
    std::thread::spawn(move || {
        for message in messages {
            std::thread::sleep(std::time::Duration::from_millis(send_interval_ms));
            if tx.send(message).is_err() {
                break; // Receiver dropped
            }
        }
    });
    
    let mut collected = Vec::new();
    let timeout = std::time::Duration::from_millis(collect_duration_ms);
    let start = std::time::Instant::now();
    
    while start.elapsed() < timeout {
        let remaining = timeout.saturating_sub(start.elapsed());
        match rx.recv_timeout(remaining) {
            Ok(message) => collected.push(message),
            Err(_) => break, // Timeout or channel closed
        }
    }
    
    collected
}
```

**Explanation:**

This solution demonstrates timeout-based message collection using channels. Key concepts:

1. **recv_timeout**: Blocks for at most the specified duration, returning Err if no message arrives in time
2. **Time Tracking**: Uses Instant to track elapsed time and calculate remaining timeout
3. **Graceful Termination**: Handles both timeout expiration and channel closure
4. **saturating_sub**: Prevents underflow when calculating remaining time

This pattern is useful for collecting events or messages within a time window, common in batch processing, buffering, or rate-limited systems. The timeout ensures the collector doesn't block indefinitely.