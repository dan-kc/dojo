// Splice Replace Practice
//
// Learning objectives:
// - Using Vec splice() method for range replacement
// - Understanding splice ownership and iterators
// - Working with range-based operations on Vec
//
// Run with: cargo test splice_replace

/// Use Vec::splice to replace a range of elements with new elements.
/// Implement efficient range replacement that returns replaced elements.
pub fn splice_replace<T>(
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_splice_replace_edge_cases() {
        // Replace at beginning
        let vec = vec![1, 2, 3, 4];
        let replacement = vec![10, 20];
        let (modified, replaced) = splice_replace(vec, 0, 2, replacement);
        assert_eq!(modified, vec![10, 20, 3, 4]);
        assert_eq!(replaced, vec![1, 2]);

        // Replace entire vector
        let vec = vec![1, 2, 3];
        let replacement = vec![10, 20, 30, 40];
        let (modified, replaced) = splice_replace(vec.clone(), 0, 3, replacement);
        assert_eq!(modified, vec![10, 20, 30, 40]);
        assert_eq!(replaced, vec);

        // Empty replacement
        let vec = vec![1, 2, 3, 4, 5];
        let replacement = vec![];
        let (modified, replaced) = splice_replace(vec, 1, 4, replacement);
        assert_eq!(modified, vec![1, 5]);
        assert_eq!(replaced, vec![2, 3, 4]);
    }

    #[test]
    fn test_splice_replace_strings() {
        let vec = vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];
        let replacement = vec!["x".to_string(), "y".to_string()];
        let (modified, replaced) = splice_replace(vec, 1, 3, replacement);
        
        assert_eq!(modified, vec!["a".to_string(), "x".to_string(), "y".to_string(), "d".to_string()]);
        assert_eq!(replaced, vec!["b".to_string(), "c".to_string()]);
    }
}