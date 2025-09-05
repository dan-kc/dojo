# HashMap Merging - Solution

## Solution

```rust
fn merge_hashmaps<K, V, F>(
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
            result.entry(key.clone())
                .and_modify(|existing| *existing = combine_fn(existing.clone(), value.clone()))
                .or_insert(value);
        }
    }
    
    result
}
```

## Explanation

This solution efficiently merges multiple HashMaps by:

1. **Creating a result HashMap**: Starting with an empty HashMap to accumulate all entries.

2. **Iterating through input maps**: Processing each HashMap in the input vector one by one.

3. **Using the Entry API**: For each key-value pair, we use `entry()` to get an `Entry` enum that represents either an occupied or vacant entry.

4. **Handling conflicts with `and_modify()`**: If the key already exists, we use the provided `combine_fn` to merge the existing value with the new value.

5. **Inserting new entries with `or_insert()`**: If the key doesn't exist, we simply insert the value.

**Key Rust concepts demonstrated:**
- **Entry API**: Efficient way to handle HashMap insertions and updates
- **Generic functions**: The function works with any key-value types that meet the trait bounds
- **Closure parameters**: The `combine_fn` allows flexible merging strategies
- **Trait bounds**: `K` must be hashable and cloneable, `V` must be cloneable
- **Ownership**: The function takes ownership of the input maps and returns a new map

**Performance characteristics:**
- Time complexity: O(n) where n is the total number of key-value pairs across all maps
- Space complexity: O(m) where m is the number of unique keys
- The Entry API avoids double hash lookups that would occur with separate `contains_key()` and `insert()` calls

This pattern is commonly used in data aggregation, configuration merging, and map-reduce operations.