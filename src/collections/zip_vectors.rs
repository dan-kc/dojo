// Run with: cargo test zip_vectors

/// Implement vector zipper - merge two vectors alternating elements.
/// Handle different lengths gracefully.
pub fn zip_vectors<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_zip_vectors_edge_cases() {
        // Empty vectors
        let empty1: Vec<i32> = vec![];
        let empty2: Vec<i32> = vec![];
        let result = zip_vectors(empty1, empty2);
        assert_eq!(result, Vec::<i32>::new());

        // One empty vector
        let vec1 = vec![1, 2, 3];
        let empty: Vec<i32> = vec![];
        let result = zip_vectors(vec1.clone(), empty.clone());
        assert_eq!(result, vec1);

        let result = zip_vectors(empty, vec1.clone());
        assert_eq!(result, vec1);
    }

    #[test]
    fn test_zip_vectors_strings() {
        let vec1 = vec!["a".to_string(), "c".to_string(), "e".to_string()];
        let vec2 = vec!["b".to_string(), "d".to_string()];
        let result = zip_vectors(vec1, vec2);
        assert_eq!(
            result,
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]
        );
    }
}
