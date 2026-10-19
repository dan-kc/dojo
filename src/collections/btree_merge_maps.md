# collections/btree_merge_maps

Reference implementation. Read after attempting the exercise.

```rust
fn merge_sorted_btreemaps<K, V, F>(
    maps: Vec<std::collections::BTreeMap<K, V>>,
    combine_fn: F,
) -> std::collections::BTreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
    F: Fn(V, V) -> V,
{
    let mut result = std::collections::BTreeMap::new();

    for map in maps {
        for (key, value) in map {
            result
                .entry(key)
                .and_modify(|existing: &mut V| {
                    *existing = combine_fn(existing.clone(), value.clone())
                })
                .or_insert(value);
        }
    }

    result
}
```
