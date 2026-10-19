# collections/partition_vector

Reference implementation. Read after attempting the exercise.

```rust
pub fn partition_vector<T, F>(mut vec: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    let mut matching = Vec::new();
    let mut non_matching = Vec::new();

    for item in vec.into_iter() {
        if predicate(&item) {
            matching.push(item);
        } else {
            non_matching.push(item);
        }
    }

    (matching, non_matching)
}
```
