// HashSet Duplicate Analysis Practice
//
// Learning objectives:
// - Combine HashSet and HashMap for duplicate detection
// - Track element frequencies across multiple collections
// - Use sets for efficient duplicate identification
//
// Run with: cargo test analyze_duplicates

/// Implement set-based duplicate detection with detailed reporting.
/// Return both the duplicates and their frequencies across inputs.
pub fn analyze_duplicates<T>(
    collections: Vec<Vec<T>>,
) -> (
    std::collections::HashSet<T>,
    std::collections::HashMap<T, usize>,
)
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_analyze_duplicates() {
        let collections = vec![
            vec!['a', 'b', 'c', 'a'],
            vec!['b', 'c', 'd', 'b'],
            vec!['c', 'd', 'e', 'c'],
        ];

        let (duplicates, frequencies) = analyze_duplicates(collections);
        let expected_duplicates: HashSet<char> = ['a', 'b', 'c', 'd'].iter().cloned().collect();
        assert_eq!(duplicates, expected_duplicates);

        assert_eq!(frequencies.get(&'a'), Some(&2));
        assert_eq!(frequencies.get(&'b'), Some(&3));
        assert_eq!(frequencies.get(&'c'), Some(&4));
        assert_eq!(frequencies.get(&'d'), Some(&2));
        assert_eq!(frequencies.get(&'e'), Some(&1));
    }

    #[test]
    fn test_analyze_duplicates_no_duplicates() {
        let collections = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        assert!(duplicates.is_empty());

        // All elements should have frequency 1
        for i in 1..=9 {
            assert_eq!(frequencies.get(&i), Some(&1));
        }
    }

    #[test]
    fn test_analyze_duplicates_all_same() {
        let collections = vec![
            vec!["same", "same"],
            vec!["same", "same", "same"],
            vec!["same"],
        ];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        let expected_duplicates: HashSet<&str> = ["same"].iter().cloned().collect();
        assert_eq!(duplicates, expected_duplicates);

        assert_eq!(frequencies.get(&"same"), Some(&6)); // 2 + 3 + 1 = 6 total
    }

    #[test]
    fn test_analyze_duplicates_empty_collections() {
        let collections: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        assert!(duplicates.is_empty());
        assert!(frequencies.is_empty());
    }

    #[test]
    fn test_analyze_duplicates_mixed_empty() {
        let collections = vec![vec![1, 1, 2], vec![], vec![2, 3, 3]];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        let expected_duplicates: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(duplicates, expected_duplicates);

        assert_eq!(frequencies.get(&1), Some(&2));
        assert_eq!(frequencies.get(&2), Some(&2)); // once in first, once in third
        assert_eq!(frequencies.get(&3), Some(&2));
    }

    #[test]
    fn test_analyze_duplicates_single_collection() {
        let collections = vec![vec!['x', 'y', 'x', 'z', 'y', 'y']];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        let expected_duplicates: HashSet<char> = ['x', 'y'].iter().cloned().collect();
        assert_eq!(duplicates, expected_duplicates);

        assert_eq!(frequencies.get(&'x'), Some(&2));
        assert_eq!(frequencies.get(&'y'), Some(&3));
        assert_eq!(frequencies.get(&'z'), Some(&1));
    }

    #[test]
    fn test_analyze_duplicates_strings() {
        let collections = vec![
            vec![
                "hello".to_string(),
                "world".to_string(),
                "hello".to_string(),
            ],
            vec!["world".to_string(), "rust".to_string()],
            vec!["rust".to_string(), "rust".to_string(), "code".to_string()],
        ];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        let expected_duplicates: HashSet<String> = ["hello", "world", "rust"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(duplicates, expected_duplicates);

        assert_eq!(frequencies.get(&"hello".to_string()), Some(&2));
        assert_eq!(frequencies.get(&"world".to_string()), Some(&2));
        assert_eq!(frequencies.get(&"rust".to_string()), Some(&3));
        assert_eq!(frequencies.get(&"code".to_string()), Some(&1));
    }

    #[test]
    fn test_analyze_duplicates_large_numbers() {
        let collections = vec![
            vec![1000000, 2000000, 1000000],
            vec![2000000, 3000000],
            vec![1000000, 3000000, 4000000],
        ];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        let expected_duplicates: HashSet<i32> =
            [1000000, 2000000, 3000000].iter().cloned().collect();
        assert_eq!(duplicates, expected_duplicates);

        assert_eq!(frequencies.get(&1000000), Some(&3));
        assert_eq!(frequencies.get(&2000000), Some(&2));
        assert_eq!(frequencies.get(&3000000), Some(&2));
        assert_eq!(frequencies.get(&4000000), Some(&1));
    }

    #[test]
    fn test_analyze_duplicates_many_collections() {
        let collections = vec![vec![1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];

        let (duplicates, frequencies) = analyze_duplicates(collections);

        // Only elements that appear more than once across all collections
        let expected_duplicates: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        assert_eq!(duplicates, expected_duplicates);

        assert_eq!(frequencies.get(&1), Some(&2));
        assert_eq!(frequencies.get(&2), Some(&2));
        assert_eq!(frequencies.get(&3), Some(&2));
        assert_eq!(frequencies.get(&4), Some(&2));
        assert_eq!(frequencies.get(&5), Some(&1));
    }
}
