# collections/btree_stepped_union

Reference implementation. Read after attempting the exercise.

```rust
fn stepped_union<T>(sets: Vec<std::collections::BTreeSet<T>>) -> Vec<std::collections::BTreeSet<T>>
where
    T: Ord + Clone,
{
    if sets.is_empty() {
        return vec![];
    }

    let mut results = Vec::new();
    let mut current_union = std::collections::BTreeSet::new();

    for set in sets {
        current_union = current_union.union(&set).cloned().collect();

        results.push(current_union.clone());
    }

    results
}
```
