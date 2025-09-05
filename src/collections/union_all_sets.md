# HashSet Union Operations Solution

## Implementation

```rust
pub fn union_all_sets<T>(sets: Vec<std::collections::HashSet<T>>) -> std::collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut result = std::collections::HashSet::new();
    
    for set in sets {
        for item in set {
            result.insert(item);
        }
    }
    
    result
}
```

## Alternative Implementation (Using extend)

```rust
pub fn union_all_sets<T>(sets: Vec<std::collections::HashSet<T>>) -> std::collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut result = std::collections::HashSet::new();
    
    for set in sets {
        result.extend(set);
    }
    
    result
}
```

## Explanation

This solution computes the union of multiple sets:

1. **Union operation**: Creates a set containing all unique elements from all input sets
2. **Automatic deduplication**: HashSet naturally eliminates duplicates across all sets
3. **Iterator consumption**: Uses `extend()` for efficient batch insertion
4. **Memory efficiency**: Transfers ownership from input sets to avoid cloning

## Key Learning Points

- **Set union**: Mathematical union operation (A ∪ B ∪ C...)
- **HashSet operations**: `extend()` provides efficient bulk insertion
- **Ownership transfer**: Consuming input sets reduces memory overhead
- **Natural deduplication**: HashSet automatically handles duplicates

## Rust Concepts Demonstrated

- HashSet construction and manipulation
- Iterator-based collection processing
- Generic functions with trait bounds
- Ownership transfer patterns
- Set theory operations in practice