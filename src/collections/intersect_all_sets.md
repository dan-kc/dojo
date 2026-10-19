# collections/intersect_all_sets

Reference implementation. Read after attempting the exercise.

```rust
pub fn intersect_all_sets<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> std::collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    sets.into_iter()
        .reduce(|acc, set| acc.intersection(&set).cloned().collect())
        .unwrap_or_default()
}
```
