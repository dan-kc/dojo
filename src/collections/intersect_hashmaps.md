# collections/intersect_hashmaps

Reference implementation. Read after attempting the exercise.

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
