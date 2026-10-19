# collections/analyze_duplicates

Reference implementation. Read after attempting the exercise.

```rust
pub fn analyze_duplicates<T>(
    collections: Vec<Vec<T>>,
) -> (
    std::collections::HashSet<T>,
    std::collections::HashMap<T, usize>,
)
where
    T: Clone + std::hash::Hash + Eq,
{
    use std::collections::{HashMap, HashSet};
    let mut count = HashMap::new();
    let mut set = HashSet::new();
    for el in collections.into_iter().flatten() {
        let count = count.entry(el.clone()).or_default();
        *count += 1;

        if *count > 1 {
            set.insert(el);
        }
    }

    (set, count)
}
```
