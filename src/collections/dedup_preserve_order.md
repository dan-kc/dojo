# collections/dedup_preserve_order

Reference implementation. Read after attempting the exercise.

```rust
pub fn dedup_preserve_order<T>(vec: Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    for item in vec {
        if seen.insert(item.clone()) {
            result.push(item);
        }
    }

    result
}
```
