// Order-Preserving Deduplication Practice
//
// Learning objectives:
// - Using HashSet for efficient deduplication
// - Preserving order during deduplication
// - Working with Vec retain operations
//
// Run with: cargo test dedup_preserve_order

/// Implement an efficient deduplication that preserves order of first occurrence.
/// Use Vec operations to achieve O(n) complexity where possible.
pub fn dedup_preserve_order<T>(vec: Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + std::hash::Hash,
{
    todo!("Implement order-preserving deduplication")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_dedup_strings() {
        let vec = vec!["a".to_string(), "b".to_string(), "a".to_string(), "c".to_string(), "b".to_string()];
        let result = dedup_preserve_order(vec);
        assert_eq!(result, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    }

    #[test]
    fn test_dedup_already_unique() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = dedup_preserve_order(vec.clone());
        assert_eq!(result, vec);
    }
}