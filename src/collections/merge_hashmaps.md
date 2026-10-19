# collections/merge_hashmaps

Reference implementation. Read after attempting the exercise.

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
            result
                .entry(key.clone())
                .and_modify(|existing: &mut V| {
                    *existing = combine_fn(existing.clone(), value.clone())
                })
                .or_insert(value);
        }
    }

    result
}
```
