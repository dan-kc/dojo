# collections/btree_range_query

Reference implementation. Read after attempting the exercise.

```rust
fn range_query<K, V>(map: &std::collections::BTreeMap<K, V>, start: &K, end: &K) -> Vec<(K, V)>
where
    K: Ord + Clone,
    V: Clone,
{
    if start > end {
        return Vec::new();
    }
    map.range(start..=end)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
```
