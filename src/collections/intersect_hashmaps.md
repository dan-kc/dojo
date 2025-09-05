# HashMap Intersection Solution

## Implementation

```rust
pub fn intersect_hashmaps<K, V>(
    map1: std::collections::HashMap<K, V>,
    map2: &std::collections::HashMap<K, V>,
) -> std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    let mut intersection = std::collections::HashMap::new();
    
    for (key, value) in map1 {
        if map2.contains_key(&key) {
            intersection.insert(key, value);
        }
    }
    
    intersection
}
```

## More Efficient Implementation

```rust
pub fn intersect_hashmaps<K, V>(
    map1: std::collections::HashMap<K, V>,
    map2: &std::collections::HashMap<K, V>,
) -> std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    // Choose the smaller map to iterate over
    if map1.len() <= map2.len() {
        map1.into_iter()
            .filter(|(key, _)| map2.contains_key(key))
            .collect()
    } else {
        map2.iter()
            .filter_map(|(key, _)| {
                map1.get(key).map(|value| (key.clone(), value.clone()))
            })
            .collect()
    }
}
```

## Explanation

This solution computes the intersection of two HashMaps, preserving values from the first map:

1. **Key intersection**: Only includes keys that exist in both maps
2. **Value preference**: Uses values from the first map when keys match
3. **Ownership handling**: Consumes the first map, borrows the second
4. **Efficiency optimization**: Iterates over the smaller map in the advanced version

## Key Learning Points

- **Set operations**: Intersection is a fundamental operation on key-value collections
- **Asymmetric operations**: The order of parameters matters for value selection
- **Performance optimization**: Iterating over the smaller collection reduces comparisons
- **Iterator patterns**: Using `filter()` and `filter_map()` for conditional collection

## Alternative Functional Implementation

```rust
pub fn intersect_hashmaps<K, V>(
    map1: std::collections::HashMap<K, V>,
    map2: &std::collections::HashMap<K, V>,
) -> std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    map1.into_iter()
        .filter(|(key, _)| map2.contains_key(key))
        .collect()
}
```

## Use Cases

- **Data merging**: Finding common keys between datasets
- **Permission systems**: Intersecting user permissions with resource requirements
- **Configuration overlap**: Finding common settings between configurations
- **Cache validation**: Checking which cached items are still relevant

## Rust Concepts Demonstrated

- HashMap iteration and filtering operations
- Iterator methods (`filter()`, `filter_map()`, `collect()`)
- Ownership patterns (consuming vs borrowing)
- Generic functions with multiple trait bounds
- Performance considerations in collection operations
- Functional programming patterns with collections