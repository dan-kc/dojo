# concurrency/parallel_reduce

Reference implementation. Read after attempting the exercise.

```rust
pub fn parallel_reduce<T: Send, F: Fn(T, T) -> T + Sync>(
    items: Vec<T>,
    workers: std::num::NonZeroUsize,
    identity: T,
    reduce: F,
) -> T {
    if items.is_empty() {
        return identity;
    }
    let count = workers.get().min(items.len());
    let size = items.len().div_ceil(count);
    let mut input = items.into_iter();
    let parts = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..count {
            let batch: Vec<_> = input.by_ref().take(size).collect();
            if batch.is_empty() {
                break;
            }
            let reduce = &reduce;
            handles.push(scope.spawn(move || batch.into_iter().reduce(reduce).unwrap()));
        }
        handles.into_iter().map(|h| h.join()).collect::<Vec<_>>()
    });
    parts
        .into_iter()
        .map(|r| r.expect("worker panicked"))
        .fold(identity, reduce)
}
```
