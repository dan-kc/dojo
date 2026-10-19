# collections/hashmap_transform

Reference implementation. Read after attempting the exercise.

```rust
pub fn transform_hashmap<K1, V1, K2, V2, FK, FV, FC>(
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
    V2: Clone,
{
    let mut result = std::collections::HashMap::new();

    for (key, value) in map {
        let new_key = key_fn(key);
        let new_value = value_fn(value);

        result
            .entry(new_key)
            .and_modify(|existing_value: &mut V2| {
                *existing_value = combine_fn(existing_value.clone(), new_value.clone());
            })
            .or_insert(new_value);
    }

    result
}
```
