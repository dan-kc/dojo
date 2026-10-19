# collections/group_by

Reference implementation. Read after attempting the exercise.

```rust
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut res = std::collections::HashMap::new();
    for item in items {
        let key = key_fn(&item);
        res.entry(key).or_insert(Vec::new()).push(item);
    }

    return res;
}
```
