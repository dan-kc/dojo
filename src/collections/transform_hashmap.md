# HashMap Transformation - Solution

## Solution

```rust
fn transform_hashmap<K1, V1, K2, V2, FK, FV, FC>(
    map: std::collections::HashMap<K1, V1>,
    key_fn: FK,
    value_fn: FV,
    combine_fn: FC,
) -> std::collections::HashMap<K2, V2>
where
    K2: std::hash::Hash + Eq,
    FK: Fn(K1) -> K2,
    FV: Fn(V1) -> V2,
    FC: Fn(V2, V2) -> V2,
{
    let mut result = std::collections::HashMap::new();
    
    for (key, value) in map {
        let new_key = key_fn(key);
        let new_value = value_fn(value);
        
        result.entry(new_key)
            .and_modify(|existing| *existing = combine_fn(existing.clone(), new_value.clone()))
            .or_insert(new_value);
    }
    
    result
}
```

## Explanation

This solution transforms both keys and values of a HashMap while handling potential key collisions:

1. **Generic transformation**: Uses three closure parameters - one for keys, one for values, and one for combining values when keys collide.

2. **Key transformation**: Applies `key_fn` to transform each original key to a new key type.

3. **Value transformation**: Applies `value_fn` to transform each original value to a new value type.

4. **Collision handling**: When the transformed key already exists, uses `combine_fn` to merge the existing value with the new value.

5. **Entry API usage**: Uses `entry()`, `and_modify()`, and `or_insert()` for efficient HashMap manipulation.

**Key Rust concepts demonstrated:**
- **Advanced generics**: Multiple generic type parameters with different purposes
- **Closure parameters**: Three different closures with different signatures
- **Trait bounds**: `K2` must implement `Hash + Eq` for HashMap keys
- **Entry API**: Efficient handling of insertions and updates
- **Type transformation**: Converting from one set of types to another

**Common use cases:**
- Normalizing keys (e.g., converting to lowercase)
- Changing data representations (e.g., strings to enums)
- Aggregating data by transformed keys
- Case-insensitive maps
- Data preprocessing and cleanup

**Performance characteristics:**
- Time complexity: O(n) where n is the number of entries
- Space complexity: O(m) where m is the number of unique transformed keys
- Handles collisions efficiently without double lookups