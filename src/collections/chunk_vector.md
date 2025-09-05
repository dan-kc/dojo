# Vector Chunking Solution

## Implementation

```rust
pub fn chunk_vector<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if chunk_size == 0 {
        return vec![];
    }
    
    vec.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}
```

## Explanation

This solution implements vector chunking by:

1. **Edge case handling**: Returns empty vector if chunk_size is 0
2. **Using chunks() iterator**: The built-in `chunks()` method creates an iterator over slices
3. **Converting to owned vectors**: Each slice is converted to an owned `Vec<T>` using `to_vec()`
4. **Collecting results**: All chunks are collected into a `Vec<Vec<T>>`

## Alternative Implementation (Manual Chunking)

```rust
pub fn chunk_vector<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if chunk_size == 0 {
        return vec![];
    }
    
    let mut result = Vec::new();
    let mut current_chunk = Vec::new();
    
    for item in vec {
        current_chunk.push(item);
        if current_chunk.len() == chunk_size {
            result.push(current_chunk);
            current_chunk = Vec::new();
        }
    }
    
    if !current_chunk.is_empty() {
        result.push(current_chunk);
    }
    
    result
}
```

## Key Learning Points

- **Iterator methods**: `chunks()` provides an elegant way to partition slices
- **Slice to Vec conversion**: `to_vec()` creates owned vectors from borrowed slices
- **Remainder handling**: The last chunk may be smaller than the specified size
- **Edge case consideration**: Zero chunk size needs special handling

## Rust Concepts Demonstrated

- Slice methods and iterators
- Vector cloning and ownership transfer
- Iterator transformations with `map()` and `collect()`
- Generic functions with Clone trait bounds
- Edge case handling in algorithms