# Find Unique Elements Solution

## Implementation

```rust
pub fn find_unique_elements<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> std::collections::HashMap<T, usize>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut element_counts = std::collections::HashMap::new();
    
    // Count occurrences of each element across all sets
    for (set_index, set) in sets.iter().enumerate() {
        for element in set {
            element_counts.entry(element.clone())
                .and_modify(|(count, _)| *count += 1)
                .or_insert((1, set_index));
        }
    }
    
    // Filter to elements that appear exactly once and return their set indices
    element_counts
        .into_iter()
        .filter_map(|(element, (count, set_index))| {
            if count == 1 {
                Some((element, set_index))
            } else {
                None
            }
        })
        .collect()
}
```

## Alternative Implementation

```rust
pub fn find_unique_elements<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> std::collections::HashMap<T, usize>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut result = std::collections::HashMap::new();
    let mut seen_elements = std::collections::HashMap::new();
    
    for (set_index, set) in sets.into_iter().enumerate() {
        for element in set {
            let count = seen_elements.entry(element.clone()).or_insert(0);
            *count += 1;
            
            // Only keep elements that appear exactly once
            if *count == 1 {
                result.insert(element, set_index);
            } else if *count > 1 {
                result.remove(&element);
            }
        }
    }
    
    result
}
```

## Explanation

This solution identifies elements unique to individual sets:

1. **Frequency counting**: Tracks how many sets contain each element
2. **Index tracking**: Records which set index each unique element came from
3. **Filtering**: Only includes elements that appear in exactly one set
4. **Result mapping**: Maps unique elements to their originating set indices

## Key Learning Points

- **Element frequency analysis**: Counting occurrences across multiple collections
- **HashMap with tuple values**: Storing both count and index information
- **Filtering patterns**: Using `filter_map()` to selectively include results
- **Set uniqueness**: Elements appearing in only one set

## Rust Concepts Demonstrated

- HashMap with complex value types (tuples)
- Iterator methods (`filter_map()`, `enumerate()`)
- Entry API for counting patterns
- Conditional logic in collection operations