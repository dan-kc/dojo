// Drain and Process Practice
//
// Learning objectives:
// - Using Vec drain() method for efficient element removal
// - Processing removed elements during iteration
// - Understanding drain's range-based removal
//
// Run with: cargo test drain_and_sum

/// Use drain to efficiently remove and process elements matching a pattern.
/// Return the sum of removed elements and modify the original vector.
pub fn drain_and_sum(mut vec: Vec<i32>, min_value: i32) -> (Vec<i32>, i32) {
    todo!("Implement drain and sum operation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drain_and_sum() {
        let vec = vec![1, 5, 2, 8, 3, 7, 4];
        let (remaining, sum) = drain_and_sum(vec, 5);
        
        // Should remove elements >= 5 and sum them
        assert_eq!(remaining, vec![1, 2, 3, 4]);
        assert_eq!(sum, 5 + 8 + 7); // 20
    }

    #[test]
    fn test_drain_and_sum_edge_cases() {
        // No elements meet criteria
        let vec = vec![1, 2, 3, 4];
        let (remaining, sum) = drain_and_sum(vec.clone(), 10);
        assert_eq!(remaining, vec);
        assert_eq!(sum, 0);

        // All elements meet criteria
        let vec = vec![5, 6, 7, 8];
        let (remaining, sum) = drain_and_sum(vec, 5);
        assert!(remaining.is_empty());
        assert_eq!(sum, 5 + 6 + 7 + 8);

        // Empty vector
        let empty = vec![];
        let (remaining, sum) = drain_and_sum(empty.clone(), 0);
        assert_eq!(remaining, empty);
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_drain_negative_values() {
        let vec = vec![-5, -2, 0, 3, -1, 4];
        let (remaining, sum) = drain_and_sum(vec, 0);
        assert_eq!(remaining, vec![-5, -2, -1]);
        assert_eq!(sum, 0 + 3 + 4); // 7
    }
}