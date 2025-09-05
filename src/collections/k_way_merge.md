# K-Way Merge Solution

## Implementation

```rust
#[derive(Eq, PartialEq)]
pub struct HeapItem<T> {
    value: T,
    iterator_id: usize,
}

impl<T: Ord> Ord for HeapItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering for min-heap behavior
        other.value.cmp(&self.value)
            .then_with(|| other.iterator_id.cmp(&self.iterator_id))
    }
}

impl<T: Ord> PartialOrd for HeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct KWayMerge<T> {
    heap: std::collections::BinaryHeap<HeapItem<T>>,
    iterators: Vec<std::iter::Peekable<std::vec::IntoIter<T>>>,
}

impl<T: Ord + Clone> KWayMerge<T> {
    pub fn new(sorted_vecs: Vec<Vec<T>>) -> Self {
        let mut heap = std::collections::BinaryHeap::new();
        let mut iterators: Vec<_> = sorted_vecs
            .into_iter()
            .map(|vec| vec.into_iter().peekable())
            .collect();
        
        // Initialize heap with first element from each non-empty iterator
        for (id, iter) in iterators.iter_mut().enumerate() {
            if let Some(value) = iter.next() {
                heap.push(HeapItem {
                    value,
                    iterator_id: id,
                });
            }
        }
        
        KWayMerge { heap, iterators }
    }
}

impl<T: Ord + Clone> Iterator for KWayMerge<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(HeapItem { value, iterator_id }) = self.heap.pop() {
            // Try to get the next element from the same iterator
            if let Some(next_value) = self.iterators[iterator_id].next() {
                self.heap.push(HeapItem {
                    value: next_value,
                    iterator_id,
                });
            }
            
            Some(value)
        } else {
            None
        }
    }
}
```

## Explanation

This solution implements k-way merge using a min-heap (BinaryHeap):

1. **HeapItem ordering**: Implements reverse ordering to create min-heap behavior
2. **Initialization**: Loads first element from each iterator into the heap
3. **Iterator pattern**: Implements Iterator trait for seamless iteration
4. **Element selection**: Always selects minimum element from heap
5. **Replenishment**: When element is consumed, adds next from same iterator

The algorithm maintains sorted order by always selecting the minimum available element.

## Key Learning Points

- **Custom Ord implementations**: Control heap ordering for min-heap behavior
- **Iterator composition**: Combining multiple iterators into single sorted stream
- **Heap maintenance**: Keeping heap populated with available elements
- **K-way merge pattern**: Efficient merging of multiple sorted sequences

## Rust Concepts Demonstrated

- Custom trait implementations (Ord, PartialOrd)
- BinaryHeap for priority queue operations
- Iterator trait implementation
- Peekable iterators for lookahead
- Generic programming with trait bounds