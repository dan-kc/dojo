# collections/btree_k_extremes

Reference implementation. Read after attempting the exercise.

```rust
fn find_k_extremes<T>(set: &std::collections::BTreeSet<T>, k: usize) -> (Vec<T>, Vec<T>)
where
    T: Ord + Clone,
{
    if k == 0 || set.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let effective_k = k.min(set.len());

    let smallest: Vec<T> = set.iter().take(effective_k).cloned().collect();

    let mut largest: Vec<T> = set.iter().rev().take(effective_k).cloned().collect();

    largest.reverse();

    (smallest, largest)
}
```
