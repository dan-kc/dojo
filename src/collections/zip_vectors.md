# Vector Zipping Solution

## Implementation

```rust
pub fn zip_vectors<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(vec1.len() + vec2.len());
    let mut iter1 = vec1.into_iter();
    let mut iter2 = vec2.into_iter();
    
    loop {
        match (iter1.next(), iter2.next()) {
            (Some(a), Some(b)) => {
                result.push(a);
                result.push(b);
            }
            (Some(a), None) => {
                result.push(a);
                result.extend(iter1);
                break;
            }
            (None, Some(b)) => {
                result.push(b);
                result.extend(iter2);
                break;
            }
            (None, None) => break,
        }
    }
    
    result
}
```
or

```rust
pub fn zip_vectors<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut iter_2 = vec2.into_iter();
    let mut res = vec![];
    for (idx, el) in vec1.into_iter().enumerate() {
        res.push(el);
        if let Some(next) = iter_2.next() {
            res.push(next);
        }
    }
    res.extend(iter_2);
    res
}
```

## Explanation

This solution implements alternating vector merging by:

1. **Capacity optimization**: Pre-allocates result vector with combined capacity
2. **Parallel iteration**: Uses iterators to consume both vectors simultaneously
3. **Pattern matching**: Handles all combinations of iterator states
4. **Remainder handling**: Extends with remaining elements when one iterator is exhausted

## Key Learning Points

- **Iterator consumption**: `into_iter()` moves elements out of the vectors
- **Pattern matching**: Clean way to handle multiple Option combinations
- **Capacity management**: Pre-allocation prevents reallocations during construction
- **Iterator extension**: `extend()` efficiently adds remaining elements

## Alternative Implementation (Using zip and chain)

```rust
pub fn zip_vectors<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let min_len = vec1.len().min(vec2.len());
    let mut result = Vec::with_capacity(vec1.len() + vec2.len());
    
    for i in 0..min_len {
        result.push(vec1[i]);
        result.push(vec2[i]);
    }
    
    // Add remaining elements
    result.extend(vec1.into_iter().skip(min_len));
    result.extend(vec2.into_iter().skip(min_len));
    result
}
```

## Rust Concepts Demonstrated

- Iterator manipulation and consumption
- Pattern matching with multiple values
- Vector capacity management and optimization
- Ownership transfer through `into_iter()`
- Efficient collection construction patterns
- Handling collections of different lengths
