# Set Partitioning Solution

## Implementation

```rust
pub fn partition_set<T, F>(
    set: std::collections::HashSet<T>,
    predicate: F,
) -> (std::collections::HashSet<T>, std::collections::HashSet<T>)
where
    T: Clone + std::hash::Hash + Eq,
    F: Fn(&T) -> bool,
{
    let mut matching = std::collections::HashSet::new();
    let mut non_matching = std::collections::HashSet::new();
    
    for item in set {
        if predicate(&item) {
            matching.insert(item);
        } else {
            non_matching.insert(item);
        }
    }
    
    (matching, non_matching)
}
```

## Alternative Implementation (Using partition)

```rust
pub fn partition_set<T, F>(
    set: std::collections::HashSet<T>,
    predicate: F,
) -> (std::collections::HashSet<T>, std::collections::HashSet<T>)
where
    T: Clone + std::hash::Hash + Eq,
    F: Fn(&T) -> bool,
{
    let (matching, non_matching): (Vec<T>, Vec<T>) = set
        .into_iter()
        .partition(predicate);
    
    (
        matching.into_iter().collect(),
        non_matching.into_iter().collect()
    )
}
```

## Explanation

This solution partitions a set based on a predicate function:

1. **Predicate testing**: Tests each element against the provided condition
2. **Conditional placement**: Elements go into appropriate result set based on test
3. **Ownership transfer**: Consumes input set and transfers elements to results
4. **Functional approach**: Uses iterator `partition()` for clean separation

## Key Learning Points

- **Set partitioning**: Dividing sets based on conditional logic
- **Predicate functions**: Using closures for flexible filtering criteria
- **Iterator partition**: Built-in method for splitting collections
- **Ownership patterns**: Moving elements between collections efficiently

## Use Cases

- **Data classification**: Separating valid from invalid records
- **Filtering operations**: Creating include/exclude sets
- **Conditional processing**: Handling different element types differently

## Rust Concepts Demonstrated

- Generic functions with closure parameters
- Iterator methods (`partition()`)
- Conditional logic with predicates
- Set construction from iterators
- Ownership transfer in collection operations