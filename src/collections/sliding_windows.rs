// Sliding Window Practice
//
// Learning objectives:
// - Using Vec windows() method
// - Working with sliding window algorithms
// - Handling edge cases in window operations
//
// Run with: cargo test sliding_windows

/// Implement sliding window operations on a vector.
/// Return all windows of the specified size as separate vectors.
pub fn sliding_windows<T>(vec: Vec<T>, window_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    todo!("Implement sliding windows")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_sliding_windows_edge_cases() {
        // Empty vector
        let empty: Vec<i32> = vec![];
        let windows = sliding_windows(empty, 2);
        assert_eq!(windows, Vec::<Vec<i32>>::new());

        // Window size equals vector size
        let vec = vec![1, 2, 3];
        let windows = sliding_windows(vec.clone(), 3);
        assert_eq!(windows, vec![vec]);

        // Window size larger than vector
        let vec = vec![1, 2];
        let windows = sliding_windows(vec, 5);
        assert!(windows.is_empty());
    }

    #[test]
    fn test_sliding_windows_strings() {
        let vec = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        let windows = sliding_windows(vec, 2);
        assert_eq!(windows, vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["c".to_string(), "d".to_string()]
        ]);
    }
}