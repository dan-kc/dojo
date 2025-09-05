// Vec Operations Practice
//
// Learning Objectives:
// - Master Vec creation, insertion, and removal operations
// - Use drain, extend, append, and split operations
// - Practice with_capacity and memory management
// - Work with Vec slicing and indexing safely
// - Understand Vec deduplication and sorting
//
// Run with: cargo test --bin vec_operations

/// Implement a function that efficiently merges multiple sorted vectors
/// into a single sorted vector using extend and sort operations.
fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    todo!("Implement merge_sorted_vectors")
}

/// Remove all elements from a vector that satisfy a predicate,
/// returning both the remaining elements and the removed elements.
fn partition_vector<T, F>(mut vec: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    todo!("Implement partition_vector using drain_filter or manual partitioning")
}

/// Implement an efficient deduplication that preserves order of first occurrence.
/// Use Vec operations to achieve O(n) complexity where possible.
fn dedup_preserve_order<T>(vec: Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + std::hash::Hash,
{
    todo!("Implement order-preserving deduplication")
}

/// Split a vector into chunks of specified size, handling remainder chunk.
/// Return a Vec of Vecs, where the last chunk may be smaller.
fn chunk_vector<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    todo!("Implement vector chunking")
}

/// Implement sliding window operations on a vector.
/// Return all windows of the specified size as separate vectors.
fn sliding_windows<T>(vec: Vec<T>, window_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    todo!("Implement sliding windows")
}

/// Rotate vector elements efficiently using Vec operations.
/// Positive n rotates right, negative n rotates left.
fn rotate_vector<T>(mut vec: Vec<T>, n: isize) -> Vec<T> {
    todo!("Implement vector rotation")
}

/// Implement in-place vector reversal in chunks.
/// Reverse every chunk of specified size within the vector.
fn reverse_chunks<T>(mut vec: Vec<T>, chunk_size: usize) -> Vec<T> {
    todo!("Implement chunked reversal")
}

/// Use drain to efficiently remove and process elements matching a pattern.
/// Return the sum of removed elements and modify the original vector.
fn drain_and_sum(mut vec: Vec<i32>, min_value: i32) -> (Vec<i32>, i32) {
    todo!("Implement drain and sum operation")
}

/// Implement vector zipper - merge two vectors alternating elements.
/// Handle different lengths gracefully.
fn zip_vectors<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    todo!("Implement vector zipper")
}

/// Use Vec::splice to replace a range of elements with new elements.
/// Implement efficient range replacement that returns replaced elements.
fn splice_replace<T>(
    mut vec: Vec<T>,
    range_start: usize,
    range_end: usize,
    replacement: Vec<T>,
) -> (Vec<T>, Vec<T>)
where
    T: Clone,
{
    todo!("Implement splice replacement")
}

/// Create a custom Vec wrapper that tracks capacity changes and operations.
struct VecTracker<T> {
    vec: Vec<T>,
    capacity_changes: usize,
    operations_count: usize,
}

impl<T> VecTracker<T> {
    fn new() -> Self {
        todo!("Implement new")
    }

    fn with_capacity(capacity: usize) -> Self {
        todo!("Implement with_capacity")
    }

    fn push(&mut self, item: T) {
        todo!("Implement tracked push")
    }

    fn pop(&mut self) -> Option<T> {
        todo!("Implement tracked pop")
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!("Implement tracked extend")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_vectors() {
        let vectors = vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
        ];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        
        // Test with empty vectors
        let vectors = vec![vec![], vec![1, 2], vec![]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![1, 2]);
        
        // Test with single vector
        let vectors = vec![vec![5, 10, 15]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![5, 10, 15]);
    }

    #[test]
    fn test_partition_vector() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let (evens, odds) = partition_vector(vec, |&x| x % 2 == 0);
        
        assert_eq!(evens, vec![2, 4, 6, 8, 10]);
        assert_eq!(odds, vec![1, 3, 5, 7, 9]);
        
        // Test with all matching
        let vec = vec![2, 4, 6];
        let (matching, non_matching) = partition_vector(vec, |&x| x % 2 == 0);
        assert_eq!(matching, vec![2, 4, 6]);
        assert!(non_matching.is_empty());
    }

    #[test]
    fn test_dedup_preserve_order() {
        let vec = vec![1, 2, 2, 3, 1, 4, 3, 5];
        let result = dedup_preserve_order(vec);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        
        let vec = vec![1, 1, 1];
        let result = dedup_preserve_order(vec);
        assert_eq!(result, vec![1]);
        
        let empty: Vec<i32> = vec![];
        let result = dedup_preserve_order(empty);
        assert!(result.is_empty());
    }

    #[test]
    fn test_chunk_vector() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let chunks = chunk_vector(vec, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        
        let vec = vec![1, 2, 3, 4, 5];
        let chunks = chunk_vector(vec, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5]]);
        
        let vec = vec![1];
        let chunks = chunk_vector(vec, 3);
        assert_eq!(chunks, vec![vec![1]]);
    }

    #[test]
    fn test_sliding_windows() {
        let vec = vec![1, 2, 3, 4, 5];
        let windows = sliding_windows(vec, 3);
        assert_eq!(windows, vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]);
        
        let vec = vec![1, 2];
        let windows = sliding_windows(vec, 3);
        assert!(windows.is_empty()); // Not enough elements
        
        let vec = vec![1, 2, 3];
        let windows = sliding_windows(vec, 1);
        assert_eq!(windows, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_rotate_vector() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = rotate_vector(vec.clone(), 2);
        assert_eq!(result, vec![4, 5, 1, 2, 3]);
        
        let result = rotate_vector(vec.clone(), -2);
        assert_eq!(result, vec![3, 4, 5, 1, 2]);
        
        let result = rotate_vector(vec.clone(), 0);
        assert_eq!(result, vec);
        
        let result = rotate_vector(vec.clone(), 5); // Full rotation
        assert_eq!(result, vec);
    }

    #[test]
    fn test_reverse_chunks() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = reverse_chunks(vec, 3);
        assert_eq!(result, vec![3, 2, 1, 6, 5, 4, 8, 7]); // [3,2,1] [6,5,4] [8,7]
        
        let vec = vec![1, 2, 3, 4];
        let result = reverse_chunks(vec, 2);
        assert_eq!(result, vec![2, 1, 4, 3]);
        
        let vec = vec![1];
        let result = reverse_chunks(vec, 3);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_drain_and_sum() {
        let vec = vec![1, 5, 2, 8, 3, 7, 4];
        let (remaining, sum) = drain_and_sum(vec, 5);
        
        // Should remove elements >= 5 and sum them
        assert_eq!(remaining, vec![1, 2, 3, 4]);
        assert_eq!(sum, 5 + 8 + 7); // 20
    }

    #[test]
    fn test_zip_vectors() {
        let vec1 = vec![1, 3, 5];
        let vec2 = vec![2, 4, 6];
        let result = zip_vectors(vec1, vec2);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
        
        let vec1 = vec![1, 3, 5, 7];
        let vec2 = vec![2, 4];
        let result = zip_vectors(vec1, vec2);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 7]);
        
        let vec1 = vec![1];
        let vec2 = vec![2, 4, 6, 8];
        let result = zip_vectors(vec1, vec2);
        assert_eq!(result, vec![1, 2, 4, 6, 8]);
    }

    #[test]
    fn test_splice_replace() {
        let vec = vec![1, 2, 3, 4, 5];
        let replacement = vec![10, 20, 30];
        let (modified, replaced) = splice_replace(vec, 1, 4, replacement);
        
        assert_eq!(modified, vec![1, 10, 20, 30, 5]);
        assert_eq!(replaced, vec![2, 3, 4]);
        
        // Test replacing at end
        let vec = vec![1, 2, 3];
        let replacement = vec![10];
        let (modified, replaced) = splice_replace(vec, 2, 3, replacement);
        assert_eq!(modified, vec![1, 2, 10]);
        assert_eq!(replaced, vec![3]);
    }

    #[test]
    fn test_vec_tracker() {
        let mut tracker = VecTracker::with_capacity(2);
        assert_eq!(tracker.capacity_changes(), 0); // Initial capacity doesn't count
        
        tracker.push(1);
        tracker.push(2);
        assert_eq!(tracker.operations_count(), 2);
        assert_eq!(tracker.capacity_changes(), 0); // No reallocation yet
        
        tracker.push(3); // Should trigger reallocation
        assert_eq!(tracker.capacity_changes(), 1);
        
        tracker.extend(vec![4, 5, 6, 7]); // Might trigger another reallocation
        assert!(tracker.operations_count() >= 3);
        
        let popped = tracker.pop();
        assert_eq!(popped, Some(7));
        
        let final_vec = tracker.into_vec();
        assert_eq!(final_vec, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_performance_characteristics() {
        // Test that with_capacity reduces reallocations
        let mut tracker_with_capacity = VecTracker::with_capacity(1000);
        let mut tracker_without_capacity = VecTracker::new();
        
        for i in 0..1000 {
            tracker_with_capacity.push(i);
            tracker_without_capacity.push(i);
        }
        
        // Pre-allocated version should have fewer capacity changes
        assert!(tracker_with_capacity.capacity_changes() < tracker_without_capacity.capacity_changes());
    }

    #[test]
    fn test_edge_cases() {
        // Test empty vector operations
        let empty: Vec<i32> = vec![];
        assert_eq!(merge_sorted_vectors(vec![empty.clone()]), Vec::<i32>::new());
        assert_eq!(chunk_vector(empty.clone(), 5), Vec::<Vec<i32>>::new());
        assert_eq!(sliding_windows(empty.clone(), 2), Vec::<Vec<i32>>::new());
        
        // Test single element vectors
        let single = vec![42];
        assert_eq!(rotate_vector(single.clone(), 1), single);
        assert_eq!(reverse_chunks(single.clone(), 1), single);
    }
}