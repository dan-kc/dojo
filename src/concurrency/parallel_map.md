# concurrency/parallel_map

Reference implementation. Read after attempting the exercise.

```rust
pub fn parallel_map<T: Send, R: Send, F: Fn(T) -> R + Sync>(
    items: Vec<T>,
    workers: std::num::NonZeroUsize,
    f: F,
) -> Vec<R> {
    if items.is_empty() {
        return vec![];
    }
    let count = workers.get().min(items.len());
    let size = items.len().div_ceil(count);
    let mut input = items.into_iter();
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..count {
            let batch: Vec<_> = input.by_ref().take(size).collect();
            if batch.is_empty() {
                break;
            }
            let f = &f;
            handles.push(scope.spawn(move || batch.into_iter().map(f).collect::<Vec<R>>()));
        }
        let results: Vec<_> = handles.into_iter().map(|h| h.join()).collect();
        results
            .into_iter()
            .flat_map(|r| r.expect("worker panicked"))
            .collect()
    })
}
```
