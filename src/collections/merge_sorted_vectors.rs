// cargo test merge_sorted_vectors
/// Implement a function that efficiently merges multiple sorted vectors
/// into a single sorted vector
#[allow(unused_variables, dead_code)]
fn merge_sorted_vectors(vectors: Vec<Vec<i32>>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_vectors() {
        let vectors = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_with_empty_vectors() {
        let vectors = vec![vec![], vec![1, 2], vec![]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_single_vector() {
        let vectors = vec![vec![5, 10, 15]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![5, 10, 15]);
    }

    #[test]
    fn test_overlapping_ranges() {
        let vectors = vec![vec![1, 3, 5], vec![2, 4, 6], vec![1, 2, 7, 8]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![1, 1, 2, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_empty_input() {
        let vectors: Vec<Vec<i32>> = vec![];
        let result = merge_sorted_vectors(vectors);
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_elements() {
        let vectors = vec![vec![9], vec![1], vec![5]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![1, 5, 9]);
        //          left: [5, 1, 9]
        // right: [1, 5, 9]
    }

    #[test]
    fn test_duplicates() {
        let vectors = vec![vec![1, 1, 2], vec![1, 2, 3], vec![2, 3, 3]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![1, 1, 1, 2, 2, 2, 3, 3, 3]);
    }

    #[test]
    fn test_negative_numbers() {
        let vectors = vec![vec![-5, -1, 3], vec![-3, 0, 2], vec![-10, 1, 4]];
        let result = merge_sorted_vectors(vectors);
        assert_eq!(result, vec![-10, -5, -3, -1, 0, 1, 2, 3, 4]);
    }
}
