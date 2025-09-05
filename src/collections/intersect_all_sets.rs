// HashSet Intersection Operations Practice
//
// Learning objectives:
// - Using HashSet intersection operations
// - Finding common elements across multiple sets
// - Handling edge cases with empty inputs
//
// Run with: cargo test intersect_all_sets

/// Find elements that appear in all input sets (intersection of all sets).
/// Handle empty input gracefully.
pub fn intersect_all_sets<T>(sets: Vec<std::collections::HashSet<T>>) -> std::collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement intersection of multiple sets")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_intersect_all_sets() {
        let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4, 5].iter().cloned().collect();
        let set3: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
        
        let result = intersect_all_sets(vec![set1, set2, set3]);
        let expected: HashSet<i32> = [3, 4].iter().cloned().collect();
        
        assert_eq!(result, expected);
        
        // Test with no common elements
        let set1: HashSet<i32> = [1, 2].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4].iter().cloned().collect();
        let result = intersect_all_sets(vec![set1, set2]);
        assert!(result.is_empty());
        
        // Test with empty input
        let result: HashSet<i32> = intersect_all_sets(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_intersect_single_set() {
        let single: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let result = intersect_all_sets(vec![single.clone()]);
        assert_eq!(result, single);
    }

    #[test]
    fn test_intersect_identical_sets() {
        let set: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();
        let result = intersect_all_sets(vec![set.clone(), set.clone(), set.clone()]);
        assert_eq!(result, set);
    }
}