# collections/jaccard_similarity

Reference implementation. Read after attempting the exercise.

```rust
pub fn jaccard_similarity<T>(
    set_a: &std::collections::HashSet<T>,
    set_b: &std::collections::HashSet<T>,
) -> f64
where
    T: std::hash::Hash + Eq,
{
    let intersection = set_a.intersection(set_b).count();
    let union = set_a.len() + set_b.len() - intersection;
    if union == 0 {
        1.0
    } else {
        intersection as f64 / union as f64
    }
}
```
