# Vector Partition Solution

## Implementation

```rust
pub fn partition_vector<T, F>(mut vec: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    let mut matching = Vec::new();
    let mut non_matching = Vec::new();
    
    for item in vec.into_iter() {
        if predicate(&item) {
            matching.push(item);
        } else {
            non_matching.push(item);
        }
    }
    
    (matching, non_matching)
}
```

## Explanation

This solution implements vector partitioning by:

1. **Creating two result vectors**: One for elements that match the predicate and one for those that don't
2. **Consuming the input vector**: Using `into_iter()` to take ownership of all elements
3. **Conditional placement**: Each element is tested against the predicate and placed in the appropriate result vector
4. **Returning both partitions**: The function returns a tuple containing both result vectors

## Key Learning Points

- **Ownership transfer**: The input vector is consumed, transferring ownership of all elements
- **Predicate functions**: Using generic function types `F: Fn(&T) -> bool` for flexible filtering
- **Memory efficiency**: Elements are moved (not cloned) into the result vectors
- **Vec consumption**: `into_iter()` consumes the vector and provides owned values

## Rust Concepts Demonstrated

- Generic functions with trait bounds
- Closures as function parameters
- Vector ownership and consumption
- Conditional logic with predicates
- Tuple return types