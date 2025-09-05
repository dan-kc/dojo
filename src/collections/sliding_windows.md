# Sliding Window Solution

## Implementation

```rust
pub fn sliding_windows<T>(vec: Vec<T>, window_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if window_size == 0 || vec.len() < window_size {
        return Vec::new();
    }
    
    vec.windows(window_size)
        .map(|window| window.to_vec())
        .collect()
}
```

## Explanation

This solution implements sliding window operations by:

1. **Edge case handling**: Returns empty vector if window_size is 0 or larger than the vector
2. **Using windows() iterator**: The built-in `windows()` method creates overlapping slices
3. **Converting to owned vectors**: Each window slice is converted to an owned `Vec<T>`
4. **Collecting all windows**: Results are collected into a vector of vectors

## Key Learning Points

- **Sliding vs chunking**: `windows()` creates overlapping slices, unlike `chunks()` which are non-overlapping
- **Window size validation**: Must check that the vector is large enough for the window size
- **Memory efficiency**: `windows()` provides views into the original data without copying until `to_vec()`
- **Iterator chaining**: Combining `windows()`, `map()`, and `collect()` for clean transformation

## Alternative Implementation (Manual)

```rust
pub fn sliding_windows<T>(vec: Vec<T>, window_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if window_size == 0 || vec.len() < window_size {
        return Vec::new();
    }
    
    let mut result = Vec::new();
    for i in 0..=(vec.len() - window_size) {
        let window = vec[i..i + window_size].to_vec();
        result.push(window);
    }
    result
}
```

## Rust Concepts Demonstrated

- Slice methods (`windows()`) and their applications
- Iterator transformations and collection
- Range-based slicing and bounds checking
- Generic programming with Clone constraints
- Memory-efficient data processing patterns