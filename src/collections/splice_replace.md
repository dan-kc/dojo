# Splice Replace Solution

## Implementation

```rust
pub fn splice_replace<T>(
    mut vec: Vec<T>,
    range_start: usize,
    range_end: usize,
    replacement: Vec<T>,
) -> (Vec<T>, Vec<T>)
where
    T: Clone,
{
    // Clamp the range to valid bounds
    let start = range_start.min(vec.len());
    let end = range_end.min(vec.len()).max(start);
    
    let replaced: Vec<T> = vec.splice(start..end, replacement).collect();
    
    (vec, replaced)
}
```

## Explanation

This solution implements range replacement using Vec's splice method:

1. **Bounds checking**: Ensures range_start and range_end are valid indices
2. **splice() method**: Replaces elements in the specified range with new elements
3. **Collecting replaced**: The splice iterator yields the replaced elements
4. **Single operation**: splice() handles both removal and insertion efficiently

## Key Learning Points

- **Vec::splice()**: Efficient method for range-based replacement operations
- **Iterator consumption**: splice() returns an iterator over the replaced elements
- **Bounds safety**: Clamping indices prevents panics from out-of-bounds access
- **Range operations**: Using range syntax (`start..end`) for slice operations

## Manual Implementation (Educational)

```rust
pub fn splice_replace<T>(
    mut vec: Vec<T>,
    range_start: usize,
    range_end: usize,
    replacement: Vec<T>,
) -> (Vec<T>, Vec<T>)
where
    T: Clone,
{
    let start = range_start.min(vec.len());
    let end = range_end.min(vec.len()).max(start);
    
    // Extract replaced elements
    let replaced = vec[start..end].to_vec();
    
    // Remove the range
    vec.drain(start..end);
    
    // Insert replacement at the start position
    for (i, item) in replacement.into_iter().enumerate() {
        vec.insert(start + i, item);
    }
    
    (vec, replaced)
}
```

## Rust Concepts Demonstrated

- Vector splice operations and range manipulation
- Iterator collection and consumption
- Bounds checking and safe indexing
- Range syntax and slice operations
- Efficient in-place collection modifications
- Generic programming with Clone constraints