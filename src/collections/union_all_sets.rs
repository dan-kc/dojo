// HashSet Union Operations Practice
//
// Learning objectives:
// - Using HashSet union operations
// - Combining multiple sets efficiently
// - Understanding set theory operations
//
// Run with: cargo test union_all_sets

/// Find all unique elements that appear in any of the input sets.
/// Use HashSet union operations efficiently.
pub fn union_all_sets<T>(sets: Vec<std::collections::HashSet<T>>) -> std::collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement union of multiple sets")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_union_all_sets() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4, 5].iter().cloned().collect();
        let set3: HashSet<i32> = [5, 6, 7].iter().cloned().collect();
        
        let result = union_all_sets(vec![set1, set2, set3]);
        let expected: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7].iter().cloned().collect();
        
        assert_eq!(result, expected);
        
        // Test with empty sets
        let empty_result = union_all_sets(vec![HashSet::<i32>::new(), HashSet::<i32>::new()]);
        assert!(empty_result.is_empty());
    }

    #[test]
    fn test_union_single_set() {
        let single: HashSet<i32> = [42].iter().cloned().collect();
        assert_eq!(union_all_sets(vec![single.clone()]), single);
    }

    #[test]
    fn test_union_empty_input() {
        let result: HashSet<i32> = union_all_sets(vec![]);
        assert!(result.is_empty());
    }
}