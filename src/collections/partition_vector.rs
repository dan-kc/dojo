// Vector Partition Practice
//
// Learning objectives:
// - Using Vec drain operations efficiently
// - Partitioning data based on predicates
// - Understanding Vec memory management during draining
//
// Run with: cargo test partition_vector

/// Remove all elements from a vector that satisfy a predicate,
/// returning both the remaining elements and the removed elements.
pub fn partition_vector<T, F>(mut vec: Vec<T>, predicate: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    todo!("Implement partition_vector using drain_filter or manual partitioning")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_partition_empty() {
        let empty: Vec<i32> = vec![];
        let (matching, non_matching) = partition_vector(empty, |&x| x > 0);
        assert!(matching.is_empty());
        assert!(non_matching.is_empty());
    }

    #[test]
    fn test_partition_strings() {
        let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string(), "code".to_string()];
        let (long, short) = partition_vector(words, |s| s.len() > 4);
        assert_eq!(long, vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(short, vec!["rust".to_string(), "code".to_string()]);
    }
}