# collections/take_batch

Reference implementation. Read after attempting the exercise.

```rust
pub fn take_batch<T>(pending: &mut Vec<T>, limit: usize) -> Vec<T> {
    let count = limit.min(pending.len());
    pending.drain(..count).collect()
}
```
