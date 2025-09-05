# Vec Operations Practice - Solution

## Solution

```rust
use std::collections::HashSet;

fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    for vec in vectors {
        result.extend(vec);
    }
    result.sort();
    result
}

fn partition_vector<T, F>(mut vec: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    let mut matching = Vec::new();
    let mut non_matching = Vec::new();
    
    for item in vec.drain(..) {
        if predicate(&item) {
            matching.push(item);
        } else {
            non_matching.push(item);
        }
    }
    
    (matching, non_matching)
}

fn dedup_preserve_order<T>(vec: Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + std::hash::Hash,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    for item in vec {
        if seen.insert(item.clone()) {
            result.push(item);
        }
    }
    
    result
}

fn chunk_vector<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if chunk_size == 0 {
        return Vec::new();
    }
    
    vec.chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn sliding_windows<T>(vec: Vec<T>, window_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if window_size == 0 || window_size > vec.len() {
        return Vec::new();
    }
    
    vec.windows(window_size)
        .map(|window| window.to_vec())
        .collect()
}

fn rotate_vector<T>(mut vec: Vec<T>, n: isize) -> Vec<T> {
    if vec.is_empty() {
        return vec;
    }
    
    let len = vec.len() as isize;
    let rotation = ((n % len) + len) % len; // Handle negative numbers
    
    if rotation != 0 {
        vec.rotate_right(rotation as usize);
    }
    
    vec
}

fn reverse_chunks<T>(mut vec: Vec<T>, chunk_size: usize) -> Vec<T> {
    if chunk_size == 0 {
        return vec;
    }
    
    for chunk in vec.chunks_mut(chunk_size) {
        chunk.reverse();
    }
    
    vec
}

fn drain_and_sum(mut vec: Vec<i32>, min_value: i32) -> (Vec<i32>, i32) {
    let mut sum = 0;
    let mut i = 0;
    
    while i < vec.len() {
        if vec[i] >= min_value {
            sum += vec.remove(i);
        } else {
            i += 1;
        }
    }
    
    (vec, sum)
}

fn zip_vectors<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    let mut iter1 = vec1.into_iter();
    let mut iter2 = vec2.into_iter();
    
    loop {
        match (iter1.next(), iter2.next()) {
            (Some(item1), Some(item2)) => {
                result.push(item1);
                result.push(item2);
            }
            (Some(item1), None) => {
                result.push(item1);
                result.extend(iter1);
                break;
            }
            (None, Some(item2)) => {
                result.push(item2);
                result.extend(iter2);
                break;
            }
            (None, None) => break,
        }
    }
    
    result
}

fn splice_replace<T>(
    mut vec: Vec<T>,
    range_start: usize,
    range_end: usize,
    replacement: Vec<T>,
) -> (Vec<T>, Vec<T>)
where
    T: Clone,
{
    let replaced: Vec<T> = vec.splice(range_start..range_end, replacement).collect();
    (vec, replaced)
}

struct VecTracker<T> {
    vec: Vec<T>,
    capacity_changes: usize,
    operations_count: usize,
}

impl<T> VecTracker<T> {
    fn new() -> Self {
        Self {
            vec: Vec::new(),
            capacity_changes: 0,
            operations_count: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            capacity_changes: 0,
            operations_count: 0,
        }
    }

    fn push(&mut self, item: T) {
        let old_capacity = self.vec.capacity();
        self.vec.push(item);
        if self.vec.capacity() > old_capacity {
            self.capacity_changes += 1;
        }
        self.operations_count += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.operations_count += 1;
        self.vec.pop()
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let old_capacity = self.vec.capacity();
        self.vec.extend(iter);
        if self.vec.capacity() > old_capacity {
            self.capacity_changes += 1;
        }
        self.operations_count += 1;
    }

    fn capacity_changes(&self) -> usize {
        self.capacity_changes
    }

    fn operations_count(&self) -> usize {
        self.operations_count
    }

    fn into_vec(self) -> Vec<T> {
        self.vec
    }
}
```

## Explanation

### Key Concepts Covered:

1. **Memory Management**: Using `with_capacity` to pre-allocate space and reduce reallocations.

2. **Drain Operations**: Using `drain(..)` to efficiently move elements out of a vector while processing them.

3. **Slicing and Windows**: Using `chunks()`, `chunks_mut()`, and `windows()` for efficient slice operations.

4. **In-place Operations**: Using `rotate_right()`, `reverse()`, and other in-place methods to avoid unnecessary allocations.

5. **Splice Operations**: Using `splice()` to efficiently replace ranges of elements.

6. **Iterator Efficiency**: Converting between iterators and vectors efficiently.

### Important Rust-Specific Considerations:

- **Move Semantics**: Many operations consume the vector (`Vec<T>`) to avoid unnecessary clones.
- **Borrowing vs Ownership**: Choosing when to take ownership vs when to borrow based on the operation needs.
- **Capacity Management**: Tracking capacity changes helps understand performance characteristics.
- **Zero-Copy Operations**: Using slices and windows where possible to avoid copying data.

### Performance Insights:

- **Pre-allocation**: `with_capacity` significantly reduces memory allocations for known sizes.
- **In-place Operations**: Methods like `rotate_right` and `reverse` work on the existing memory.
- **Drain Patterns**: Using `drain` is more efficient than repeated `remove` calls.
- **Chunking**: `chunks()` provides zero-copy slices instead of creating new vectors.

### Advanced Patterns:

- **Custom Wrappers**: The `VecTracker` shows how to wrap collections to add instrumentation.
- **Efficient Partitioning**: Using `drain` for partitioning avoids multiple passes through the data.
- **Deduplication**: Combining `HashSet` with vector operations for efficient unique filtering.