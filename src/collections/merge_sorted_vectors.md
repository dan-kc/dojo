# Merge Sorted Vectors

## Solution

```rust
fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    if vectors.is_empty() {
        return Vec::new();
    }
    
    // Calculate total capacity needed
    let total_len: usize = vectors.iter().map(|v| v.len()).sum();
    let mut result = Vec::with_capacity(total_len);
    
    // Extend with all vectors
    for vector in vectors {
        result.extend(vector);
    }
    
    // Sort the combined result
    result.sort();
    result
}
```

## Alternative Implementation with Iterator

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

## Efficient Merge for Already Sorted Inputs

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn merge_sorted_vectors_efficient(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    if vectors.is_empty() {
        return Vec::new();
    }
    
    // Use a min-heap to efficiently merge sorted vectors
    let mut heap = BinaryHeap::new();
    let mut iterators: Vec<_> = vectors.into_iter()
        .enumerate()
        .filter_map(|(i, v)| {
            let mut iter = v.into_iter();
            iter.next().map(|first| (iter, first, i))
        })
        .collect();
    
    // Initialize heap with first element from each non-empty vector
    for (iter, first, idx) in iterators.iter_mut() {
        heap.push(Reverse((*first, idx)));
    }
    
    let mut result = Vec::new();
    
    while let Some(Reverse((value, vec_idx))) = heap.pop() {
        result.push(value);
        
        // Get next element from the same vector
        if let Some(next_value) = iterators[vec_idx].0.next() {
            heap.push(Reverse((next_value, vec_idx)));
        }
    }
    
    result
}
```

## Simple Concatenation Approach

```rust
fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    
    for vector in vectors {
        result.extend(vector);
    }
    
    result.sort_unstable(); // Slightly faster than sort() for primitive types
    result
}
```

## Explanation

The merge sorted vectors function demonstrates several Vec manipulation techniques:

1. **Capacity Pre-allocation**: Calculate the total needed capacity to minimize reallocations during extend operations.

2. **Extend Operation**: `Vec::extend()` efficiently appends all elements from each input vector.

3. **Iterator Chaining**: Using `flat_map()` provides a functional approach to concatenate all vectors.

4. **Sorting**: After concatenation, a single sort operation orders all elements.

5. **Efficient Merge**: For truly sorted inputs, a heap-based merge approach maintains O(n log k) complexity where k is the number of vectors.

6. **Memory Efficiency**: The simple approaches reuse existing allocations and minimize temporary storage.

This pattern is useful for:
- Combining results from multiple sorted data sources
- Merging query results from different databases
- Combining sorted log files
- Consolidating sorted partial results from parallel processing

The trade-off is between implementation simplicity (concatenate + sort) versus algorithmic efficiency (heap-based merge for sorted inputs).