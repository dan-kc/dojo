// K-Way Merge Practice
//
// Learning Objectives:
// - Implement k-way merge using BinaryHeap
// - Practice with custom Ord implementations for heap ordering
// - Merge k sorted iterators into a single sorted iterator
// - Understand min-heap patterns with custom data structures
//
// Run with: cargo test --bin k_way_merge

/// Implement k-way merge using BinaryHeap.
/// Merge k sorted iterators into a single sorted iterator.
pub struct KWayMerge<T> {
    heap: std::collections::BinaryHeap<HeapItem<T>>,
    iterators: Vec<std::iter::Peekable<std::vec::IntoIter<T>>>,
}

#[derive(Eq, PartialEq)]
pub struct HeapItem<T> {
    value: T,
    iterator_id: usize,
}

impl<T: Ord> Ord for HeapItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("Implement ordering for min-heap (reverse of natural order)")
    }
}

impl<T: Ord> PartialOrd for HeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Clone> KWayMerge<T> {
    pub fn new(sorted_vecs: Vec<Vec<T>>) -> Self {
        todo!("Initialize k-way merge with sorted vectors")
    }
}

impl<T: Ord + Clone> Iterator for KWayMerge<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement iterator that returns elements in sorted order")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn test_k_way_merge() {
        let vec1 = vec![1, 4, 7];
        let vec2 = vec![2, 5, 8];
        let vec3 = vec![3, 6, 9];
        
        let merge = KWayMerge::new(vec![vec1, vec2, vec3]);
        let result: Vec<_> = merge.collect();
        
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        
        // Test with empty vectors
        let vec1 = vec![];
        let vec2 = vec![1, 3];
        let vec3 = vec![2];
        
        let merge = KWayMerge::new(vec![vec1, vec2, vec3]);
        let result: Vec<_> = merge.collect();
        
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_heap_item_ordering() {
        let mut heap = BinaryHeap::new();
        
        heap.push(HeapItem { value: 3, iterator_id: 0 });
        heap.push(HeapItem { value: 1, iterator_id: 1 });
        heap.push(HeapItem { value: 2, iterator_id: 2 });
        
        // Should come out in min-heap order: 1, 2, 3
        assert_eq!(heap.pop().unwrap().value, 1);
        assert_eq!(heap.pop().unwrap().value, 2);
        assert_eq!(heap.pop().unwrap().value, 3);
    }
}