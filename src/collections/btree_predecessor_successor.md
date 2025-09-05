# BTree Predecessor/Successor Solution

## Implementation

```rust
fn find_predecessor_successor<T>(
    set: &std::collections::BTreeSet<T>,
    target: &T,
) -> (Option<T>, Option<T>)
where
    T: Ord + Clone,
{
    use std::ops::Bound;
    
    // Find predecessor: largest element < target
    let predecessor = set.range((Bound::Unbounded, Bound::Excluded(target)))
        .next_back()
        .cloned();
    
    // Find successor: smallest element > target
    let successor = set.range((Bound::Excluded(target), Bound::Unbounded))
        .next()
        .cloned();
    
    (predecessor, successor)
}
```

## Explanation

This solution finds predecessor and successor efficiently using BTreeSet ranges:

1. **Predecessor query**: Uses range with excluded upper bound at target
2. **next_back()**: Gets last element in range (largest < target)
3. **Successor query**: Uses range with excluded lower bound at target
4. **next()**: Gets first element in range (smallest > target)
5. **Efficient access**: O(log n) time complexity for both operations

## Key Learning Points

- **Range queries**: Using Bound enum for flexible range specifications
- **Bidirectional iteration**: next_back() for accessing from end
- **Excluded bounds**: Finding strictly less/greater elements
- **BTreeSet efficiency**: Logarithmic time for ordered queries

## Rust Concepts Demonstrated

- BTreeSet range method with custom bounds
- std::ops::Bound enum (Unbounded, Excluded, Included)
- Bidirectional iterator methods
- Option type for nullable results
- Clone trait for value extraction