// Chunked Reversal Practice
//
// Learning objectives:
// - Working with Vec chunks_mut() for in-place operations
// - Implementing custom chunking and reversal logic
// - Understanding Vec memory layout and slicing
//
// Run with: cargo test reverse_chunks

/// Implement in-place vector reversal in chunks.
/// Reverse every chunk of specified size within the vector.
pub fn reverse_chunks<T>(mut vec: Vec<T>, chunk_size: usize) -> Vec<T> {
    todo!("Implement chunked reversal")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_reverse_chunks_edge_cases() {
        // Empty vector
        let empty: Vec<i32> = vec![];
        let result = reverse_chunks(empty.clone(), 2);
        assert_eq!(result, empty);

        // Chunk size 1 (no change)
        let vec = vec![1, 2, 3, 4];
        let result = reverse_chunks(vec.clone(), 1);
        assert_eq!(result, vec);

        // Chunk size larger than vector
        let vec = vec![1, 2, 3];
        let result = reverse_chunks(vec.clone(), 5);
        assert_eq!(result, vec![3, 2, 1]);
    }

    #[test]
    fn test_reverse_chunks_chars() {
        let vec = vec!['a', 'b', 'c', 'd', 'e', 'f'];
        let result = reverse_chunks(vec, 2);
        assert_eq!(result, vec!['b', 'a', 'd', 'c', 'f', 'e']);
    }
}