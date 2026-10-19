// cargo test --lib collections::chunk_vector::

//! Consume the vector into consecutive chunks of chunk_size items, retaining a shorter final chunk.
//! Move elements without requiring Clone. Zero chunk_size must panic, even for empty input.
//! Do not repeatedly remove from the front of a Vec; total work should be O(n).

pub fn chunk_vector<T>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    todo!()
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

#[cfg(test)]
mod ownership_tests {
    use super::*;
    #[test]
    fn moves_nonclone_values() {
        struct Item(u8);
        let chunks = chunk_vector(vec![Item(1), Item(2), Item(3)], 2);
        assert_eq!(chunks[1][0].0, 3);
    }
    #[test]
    #[should_panic]
    fn rejects_zero() {
        chunk_vector(Vec::<u8>::new(), 0);
    }
}
