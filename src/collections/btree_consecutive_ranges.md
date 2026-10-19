# collections/btree_consecutive_ranges

Reference implementation. Read after attempting the exercise.

```rust
fn find_consecutive_ranges(set: &std::collections::BTreeSet<i32>) -> Vec<(i32, i32)> {
    let mut runs: Vec<(i32, i32)> = Vec::new();
    for &value in set {
        match runs.last_mut() {
            Some((_, end)) if end.checked_add(1) == Some(value) => *end = value,
            _ => runs.push((value, value)),
        }
    }
    runs
}
```
