# HashSet Duplicate Analysis Solution

## Implementation

```rust
pub fn analyze_duplicates<T>(
    collections: Vec<Vec<T>>,
) -> (
    std::collections::HashSet<T>,
    std::collections::HashMap<T, usize>,
)
where
    T: Clone + std::hash::Hash + Eq,
{
    use std::collections::{HashMap, HashSet};
    let mut count = HashMap::new();
    let mut set = HashSet::new();
    for el in collections.into_iter().flatten() {
        let count = count.entry(el.clone()).or_default();
        *count += 1;

        if *count > 1 {
            set.insert(el);
        }
    }

    (set, count)
}
```

## Explanation

This solution analyzes duplicates across multiple collections:

1. **Frequency tracking**: Counts total occurrences of each element
2. **Duplicate identification**: Elements appearing more than once are duplicates
3. **Simultaneous processing**: Builds both result structures in single pass
4. **Comprehensive analysis**: Provides both duplicate set and frequency map

## Key Learning Points

- **Multi-collection analysis**: Processing elements from multiple sources
- **Frequency counting**: Using HashMap for occurrence tracking
- **Duplicate detection**: Identifying elements with count > 1
- **Combined results**: Returning both duplicates and their frequencies

## Rust Concepts Demonstrated

- HashMap for frequency counting
- HashSet for duplicate collection
- Entry API for counting patterns
- Multi-collection processing
- Simultaneous data structure building
