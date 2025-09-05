// Group By Practice
//
// Learning objectives:
// - Using HashMap for efficient grouping operations
// - Working with Entry API for building collections
// - Implementing functional programming patterns with HashMap
//
// Run with: cargo test group_by

/// Group vector elements by a key function, returning a HashMap of groups.
/// Use Entry API for efficient grouping.
pub fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    todo!("Implement grouping using HashMap")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_by() {
        let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
        let grouped = group_by(words, |word| word.chars().next().unwrap());
        
        assert_eq!(grouped.get(&'a'), Some(&vec!["apple", "apricot"]));
        assert_eq!(grouped.get(&'b'), Some(&vec!["banana", "blueberry"]));
        assert_eq!(grouped.get(&'c'), Some(&vec!["cherry"]));
    }

    #[test]
    fn test_group_by_length() {
        let words = vec!["hi", "bye", "hello", "world", "rust", "code"];
        let grouped = group_by(words, |word| word.len());
        
        assert_eq!(grouped.get(&2), Some(&vec!["hi"]));
        assert_eq!(grouped.get(&3), Some(&vec!["bye"]));
        assert_eq!(grouped.get(&4), Some(&vec!["rust", "code"]));
        assert_eq!(grouped.get(&5), Some(&vec!["hello", "world"]));
    }

    #[test]
    fn test_group_numbers_by_parity() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let grouped = group_by(numbers, |&n| n % 2);
        
        assert_eq!(grouped.get(&0), Some(&vec![2, 4, 6, 8, 10])); // Even
        assert_eq!(grouped.get(&1), Some(&vec![1, 3, 5, 7, 9]));  // Odd
    }

    #[test]
    fn test_group_empty_vec() {
        let empty: Vec<i32> = vec![];
        let grouped = group_by(empty, |&x| x > 0);
        
        assert!(grouped.is_empty());
    }

    #[test]
    fn test_group_single_group() {
        let items = vec![1, 1, 1, 1];
        let grouped = group_by(items, |&x| x);
        
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped.get(&1), Some(&vec![1, 1, 1, 1]));
    }
}