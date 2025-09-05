# Drain and Sum Solution

## Implementation

```rust
pub fn drain_and_sum(mut vec: Vec<i32>, min_value: i32) -> (Vec<i32>, i32) {
    let mut sum = 0;
    let mut i = 0;
    
    while i < vec.len() {
        if vec[i] >= min_value {
            sum += vec.remove(i);
        } else {
            i += 1;
        }
    }
    
    (vec, sum)
}
```

## Better Implementation Using retain

```rust
pub fn drain_and_sum(mut vec: Vec<i32>, min_value: i32) -> (Vec<i32>, i32) {
    let mut sum = 0;
    
    vec.retain(|&x| {
        if x >= min_value {
            sum += x;
            false // Remove this element
        } else {
            true // Keep this element
        }
    });
    
    (vec, sum)
}
```

## Explanation

The better implementation using `retain` works by:

1. **Mutable sum tracking**: Maintains a running sum of removed elements
2. **retain() method**: Efficiently removes elements in-place based on a predicate
3. **Closure capture**: The closure captures `sum` and `min_value` from the outer scope
4. **Single pass**: Processes all elements in one iteration with O(n) complexity

## Key Learning Points

- **Vec::retain()**: More efficient than manual indexing and removal for filtering operations
- **Closure capture**: Closures can capture and modify variables from their environment
- **In-place filtering**: `retain()` modifies the vector without additional memory allocation
- **Side effects in closures**: The closure both filters and accumulates the sum

## Alternative Using drain_filter (when available)

```rust
// This would be the ideal solution when drain_filter is stabilized
pub fn drain_and_sum(mut vec: Vec<i32>, min_value: i32) -> (Vec<i32>, i32) {
    let drained: i32 = vec.drain_filter(|&mut x| x >= min_value).sum();
    (vec, drained)
}
```

## Rust Concepts Demonstrated

- Vector mutation and filtering operations
- Closure capturing with mutable references
- Efficient in-place collection modifications
- Side effects in functional programming constructs
- Iterator-based summation and collection
- Memory-efficient removal patterns