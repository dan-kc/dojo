# collections/partition_set

Reference implementation. Read after attempting the exercise.

```rust
pub fn partition_set<T, F>(
    set: std::collections::HashSet<T>,
    predicate: F,
) -> (std::collections::HashSet<T>, std::collections::HashSet<T>)
where
    T: Clone + std::hash::Hash + Eq,
    F: Fn(&T) -> bool,
{
    let mut matching = std::collections::HashSet::new();
    let mut non_matching = std::collections::HashSet::new();

    for item in set {
        if predicate(&item) {
            matching.insert(item);
        } else {
            non_matching.insert(item);
        }
    }

    (matching, non_matching)
}
```
