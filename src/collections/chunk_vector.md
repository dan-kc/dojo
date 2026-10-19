# collections/chunk_vector

Reference implementation. Read after attempting the exercise.

```rust
pub fn chunk_vector<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    assert!(chunk_size > 0);
    let mut input = vec.into_iter();
    let mut chunks = Vec::new();
    loop {
        let chunk: Vec<T> = input.by_ref().take(chunk_size).collect();
        if chunk.is_empty() {
            break;
        }
        chunks.push(chunk);
    }
    chunks
}
```
