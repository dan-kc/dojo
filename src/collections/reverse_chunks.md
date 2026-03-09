# Chunked Reversal Solution

## Implementation

```rust
pub fn reverse_chunks<T>(mut vec: Vec<T>, chunk_size: usize) -> Vec<T> {
    for chunk in vec.chunks_mut(chunk_size) {
        chunk.reverse();
    }
    vec
}
```
