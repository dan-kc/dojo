# collections/btree_split_map

Reference implementation. Read after attempting the exercise.

```rust
fn split_btree_map<K, V>(
    mut map: std::collections::BTreeMap<K, V>,
    split_key: &K,
) -> (
    std::collections::BTreeMap<K, V>,
    std::collections::BTreeMap<K, V>,
)
where
    K: Ord + Clone,
{
    let right = map.split_off(split_key);

    let left = map;

    (left, right)
}
```
