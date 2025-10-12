# Merge Sorted Vectors

## Solution

```rust
fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    // Initialize our heap with the first el in each vec
    let mut heap = std::collections::BinaryHeap::new();
    let mut iters: Vec<std::vec::IntoIter<i32>> =
        vectors.into_iter().map(|vec| vec.into_iter()).collect();

    for (idx, vector) in iters.iter_mut().enumerate() {
        if let Some(val) = vector.next() {
            heap.push(std::cmp::Reverse((val, idx)));
        }
    }

    // Loop through all elements
    let mut res = vec![];
    while !heap.is_empty() {
        let std::cmp::Reverse((val, vec_idx)) = heap.pop().unwrap();
        res.push(val);

        // add to heap from the vec we just took from
        let vecs = &mut iters;
        if let Some(next) = vecs.get_mut(vec_idx).unwrap().next() {
            heap.push(std::cmp::Reverse((next, vec_idx)))
        }
    }
    res
}
```

## Alternative Slower Implementation

```rust
fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vectors
        .into_iter()
        .flat_map(|v| v.into_iter())
        .collect();

    result.sort();
    result
}
```
