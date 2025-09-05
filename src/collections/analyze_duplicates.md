# HashSet Duplicate Analysis Solution

## Implementation

```rust
pub fn analyze_duplicates<T>(
    collections: Vec<Vec<T>>,
) -> (std::collections::HashSet<T>, std::collections::HashMap<T, usize>)
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut frequencies = std::collections::HashMap::new();
    let mut duplicates = std::collections::HashSet::new();
    
    // Count all occurrences across all collections
    for collection in collections {
        for item in collection {
            let count = frequencies.entry(item.clone()).or_insert(0);
            *count += 1;
            
            // Mark as duplicate if appears more than once
            if *count > 1 {
                duplicates.insert(item);
            }
        }
    }
    
    (duplicates, frequencies)
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