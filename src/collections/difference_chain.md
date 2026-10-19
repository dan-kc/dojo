# collections/difference_chain

Reference implementation. Read after attempting the exercise.

```rust
pub fn difference_chain<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut res = vec![];
    for sets in sets.windows(2) {
        let new_set: std::collections::HashSet<T> = sets[0].difference(&sets[1]).cloned().collect();

        res.push(new_set);
    }
    res
}
```
