# collections/btree_predecessor_successor

Reference implementation. Read after attempting the exercise.

```rust
fn find_predecessor_successor<T>(
    set: &std::collections::BTreeSet<T>,
    target: &T,
) -> (Option<T>, Option<T>)
where
    T: Ord + Clone,
{
    use std::ops::Bound;

    let predecessor = set
        .range((Bound::Unbounded, Bound::Excluded(target)))
        .next_back()
        .cloned();

    let successor = set
        .range((Bound::Excluded(target), Bound::Unbounded))
        .next()
        .cloned();

    (predecessor, successor)
}
```
