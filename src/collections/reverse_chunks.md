# Chunked Reversal Solution

## Implementation

```rust
pub fn reverse_chunks<T>(mut vec: Vec<T>, chunk_size: usize) -> Vec<T> {
    if chunk_size <= 1 || vec.is_empty() {
        if chunk_size == 0 {
            return vec;
        }
        // If chunk_size is 1, just reverse the entire vector
        if chunk_size > vec.len() {
            vec.reverse();
        }
        return vec;
    }
    
    for chunk in vec.chunks_mut(chunk_size) {
        chunk.reverse();
    }
    
    vec
}
```

## Explanation

This solution implements in-place chunked reversal by:

1. **Edge case handling**: Handles empty vectors, chunk size 0, 1, and larger than vector length
2. **Mutable chunks**: Uses `chunks_mut()` to get mutable slices of the specified size
3. **In-place reversal**: Each chunk is reversed in place using the `reverse()` method
4. **Memory efficiency**: No additional memory allocation, works directly on the input vector

## Key Learning Points

- **Mutable slicing**: `chunks_mut()` provides mutable references to non-overlapping slices
- **In-place operations**: `reverse()` modifies slices directly without additional memory
- **Chunk handling**: The last chunk may be smaller than the specified chunk_size
- **Ownership patterns**: The vector is modified and returned, maintaining ownership

## Alternative Implementation (Functional Style)

```rust
pub fn reverse_chunks<T>(vec: Vec<T>, chunk_size: usize) -> Vec<T>
where
    T: Clone,
{
    if chunk_size == 0 {
        return vec;
    }
    
    vec.chunks(chunk_size)
        .flat_map(|chunk| chunk.iter().rev().cloned())
        .collect()
}
```

## Rust Concepts Demonstrated

- Mutable slice operations (`chunks_mut()`)
- In-place vector modifications
- Iterator patterns for chunk processing
- Memory-efficient algorithms
- Edge case handling in collection operations
- Functional vs imperative programming styles