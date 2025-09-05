# BTree K Extremes Solution

## Implementation

```rust
fn find_k_extremes<T>(
    set: &std::collections::BTreeSet<T>,
    k: usize,
) -> (Vec<T>, Vec<T>)
where
    T: Ord + Clone,
{
    // Handle edge case where k is 0
    if k == 0 || set.is_empty() {
        return (Vec::new(), Vec::new());
    }
    
    // Limit k to the size of the set
    let effective_k = k.min(set.len());
    
    // Get k smallest elements (in ascending order)
    let smallest: Vec<T> = set.iter()
        .take(effective_k)
        .cloned()
        .collect();
    
    // Get k largest elements (in ascending order)
    // Use rev() to iterate from largest, then reverse the result
    let mut largest: Vec<T> = set.iter()
        .rev()
        .take(effective_k)
        .cloned()
        .collect();
    
    // Reverse to get ascending order
    largest.reverse();
    
    (smallest, largest)
}
```

## Explanation

This solution efficiently finds k smallest and largest elements:

1. **Edge case handling**: Returns empty vectors for k=0 or empty set
2. **Effective k**: Limits k to set size to avoid iterator exhaustion
3. **K smallest**: Uses forward iteration with take(k)
4. **K largest**: Uses reverse iteration, then reverses result for ascending order
5. **Efficient access**: BTreeSet's ordered nature enables O(k) access to extremes

## Key Learning Points

- **Ordered iteration**: BTreeSet maintains sorted order for efficient extreme access
- **Iterator methods**: Combining take(), rev(), and cloned() for selection
- **Edge case handling**: Gracefully handling k larger than set size
- **Result ordering**: Maintaining consistent ascending order in results

## Rust Concepts Demonstrated

- BTreeSet for ordered storage
- Iterator combinators (take, rev, cloned)
- Vector reversal for reordering
- min() method for safe bounds limiting
- Generic constraints with Ord and Clone