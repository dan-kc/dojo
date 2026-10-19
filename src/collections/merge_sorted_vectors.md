# collections/merge_sorted_vectors

Reference implementation. Read after attempting the exercise.

```rust
fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    use std::cmp::Reverse;

    let total_len = vectors.iter().fold(0, |acc, e| acc + e.len());
    let mut res = Vec::with_capacity(total_len);

    let mut min_heap = std::collections::BinaryHeap::new();
    let mut iters_enum: Vec<_> = vectors.into_iter().map(|v| v.into_iter()).collect();

    for (idx, iter) in iters_enum.iter_mut().enumerate() {
        if let Some(next) = iter.next() {
            min_heap.push((Reverse(next), idx))
        }
    }

    while !min_heap.is_empty() {
        let (rev_val, idx) = min_heap.pop().unwrap();
        res.push(rev_val.0);
        if let Some(next) = iters_enum[idx].next() {
            min_heap.push((Reverse(next), idx))
        };
    }
    res
}
```
