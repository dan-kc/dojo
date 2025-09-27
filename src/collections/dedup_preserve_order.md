# Order-Preserving Deduplication Solution

## Implementation

```rust
pub fn dedup_preserve_order<T>(vec: Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    
    for item in vec {
        if seen.insert(item.clone()) {
            result.push(item);
        }
    }
    
    result
}
```

## Explanation

This solution implements efficient order-preserving deduplication by:

1. **Using HashSet for tracking**: `HashSet::insert()` returns `true` if the item was newly inserted (not seen before)
2. **Preserving first occurrence**: Only the first occurrence of each unique element is kept
3. **Linear time complexity**: O(n) average case performance due to HashSet operations
4. **Order preservation**: Items are added to the result vector in their original order

## Key Learning Points

- **HashSet efficiency**: `insert()` method both adds and checks for existence in O(1) average time
- **Clone optimization**: We clone only when checking, but move the original into the result
- **Memory management**: The result vector grows only with unique elements
- **Trait bounds**: Requires `Hash` for HashSet, `PartialEq` for equality, and `Clone` for duplication

## Rust Concepts Demonstrated

- HashSet operations and ownership
- Generic trait bounds (`Hash`, `PartialEq`, `Clone`)
- Efficient deduplication algorithms
- Vector construction and memory management
- Conditional logic with collection operations
