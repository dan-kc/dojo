# Find Unique Elements Solution

## Implementation

```rust
pub fn find_unique_elements<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> std::collections::HashMap<T, usize>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut count: std::collections::HashMap<T, Option<usize>> = std::collections::HashMap::new();
    for (idx, set) in sets.into_iter().enumerate() {
        for val in set {
            count
                .entry(val)
                .and_modify(|curr| *curr = None)
                .or_insert(Some(idx));
        }
    }

    count.retain(|_, v| v.is_some());
    let mut res = std::collections::HashMap::new();
    for (k, v) in count {
        res.insert(k, v.unwrap());
    }

    return res;
}
```

