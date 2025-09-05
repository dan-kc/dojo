# HashMap Merging Solution

## Implementation

```rust
pub fn merge_hashmaps<K, V, F>(
    maps: Vec<std::collections::HashMap<K, V>>,
    combine_fn: F,
) -> std::collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
    F: Fn(V, V) -> V,
{
    let mut result = std::collections::HashMap::new();
    
    for map in maps {
        for (key, value) in map {
            result.entry(key).and_modify(|existing_value| {
                *existing_value = combine_fn(existing_value.clone(), value.clone());
            }).or_insert(value);
        }
    }
    
    result
}
```

## Alternative Implementation (More Efficient)

```rust
pub fn merge_hashmaps<K, V, F>(
    maps: Vec<std::collections::HashMap<K, V>>,
    combine_fn: F,
) -> std::collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
    F: Fn(V, V) -> V,
{
    let mut result = std::collections::HashMap::new();
    
    for map in maps {
        for (key, value) in map {
            match result.entry(key) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    let combined = combine_fn(entry.get().clone(), value);
                    entry.insert(combined);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(value);
                }
            }
        }
    }
    
    result
}
```

## Explanation

This solution merges multiple HashMaps using a custom combination function:

1. **Iterative merging**: Processes each map in sequence, accumulating results
2. **Entry API for conflicts**: Uses the Entry API to handle key collisions efficiently
3. **Generic combination**: The `combine_fn` parameter allows flexible value merging strategies
4. **Ownership handling**: Takes ownership of input maps to avoid unnecessary cloning

## Key Learning Points

- **Entry API patterns**: Different approaches using `and_modify()`/`or_insert()` vs explicit pattern matching
- **Generic functions**: Multiple type parameters and trait bounds for flexible APIs
- **Value combination strategies**: Addition, max, concatenation, etc., via function parameters
- **Memory efficiency**: Consuming input maps reduces cloning overhead

## Rust Concepts Demonstrated

- Generic functions with multiple type parameters and trait bounds
- HashMap Entry API for efficient conflict resolution
- Function parameters for customizable behavior
- Ownership transfer and consumption patterns
- Iterator processing over multiple collections