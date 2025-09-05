// Vector Chunking Practice
//
// Learning objectives:
// - Using Vec chunks and iteration
// - Handling remainders in chunking operations
// - Working with Vec slicing and cloning
//
// Run with: cargo test chunk_vector

/// Split a vector into chunks of specified size, handling remainder chunk.
/// Return a Vec of Vecs, where the last chunk may be smaller.
pub fn chunk_vector<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    todo!("Implement vector chunking")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_chunk_empty() {
        let empty: Vec<i32> = vec![];
        let chunks = chunk_vector(empty, 5);
        assert_eq!(chunks, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_chunk_size_one() {
        let vec = vec![1, 2, 3, 4];
        let chunks = chunk_vector(vec, 1);
        assert_eq!(chunks, vec![vec![1], vec![2], vec![3], vec![4]]);
    }

    #[test]
    fn test_chunk_larger_than_vec() {
        let vec = vec![1, 2, 3];
        let chunks = chunk_vector(vec.clone(), 5);
        assert_eq!(chunks, vec![vec]);
    }
}