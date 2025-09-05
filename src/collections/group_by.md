# Group By Solution

## Implementation

```rust
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups = std::collections::HashMap::new();
    
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    
    groups
}
```

## Alternative Implementation (Using and_modify)

```rust
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups = std::collections::HashMap::new();
    
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).and_modify(|vec| vec.push(item)).or_insert_with(|| vec![item]);
    }
    
    groups
}
```

## Explanation

This solution implements efficient grouping using HashMap and Entry API:

1. **Key function**: Extracts grouping key from each item using the provided function
2. **Entry API**: `or_insert_with(Vec::new)` creates empty vectors for new groups
3. **Direct pushing**: Adds items to the appropriate group vector
4. **Single pass**: O(n) time complexity with one iteration through items

## Key Learning Points

- **or_insert_with()**: Lazy initialization of default values using a closure
- **Functional grouping**: Using key extraction functions for flexible grouping logic
- **Entry API efficiency**: Avoiding double lookups when adding to existing groups
- **Vec operations**: Direct pushing to group vectors without intermediate collections

## Alternative with Capacity Optimization

```rust
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups = std::collections::HashMap::new();
    let item_count = items.len();
    
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(|| Vec::with_capacity(item_count)).push(item);
    }
    
    groups
}
```

## Rust Concepts Demonstrated

- HashMap Entry API (`entry()`, `or_insert_with()`)
- Closure-based lazy initialization
- Generic functions with trait bounds
- Efficient collection building patterns
- Functional programming concepts (key extraction)
- Memory optimization with capacity management