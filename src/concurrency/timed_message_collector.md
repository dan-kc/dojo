# concurrency/timed_message_collector

Reference implementation. Read after attempting the exercise.

```rust
pub fn timed_message_collector<T>(
    receiver: &std::sync::mpsc::Receiver<T>,
    deadline: std::time::Instant,
) -> Vec<T> {
    let mut out = Vec::new();
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        match receiver.recv_timeout(remaining) {
            Ok(item) => out.push(item),
            Err(_) => break,
        }
    }
    out
}
```
