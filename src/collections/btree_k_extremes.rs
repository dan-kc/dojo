// BTree K Extremes Practice
//
// Learning Objectives:
// - Find k smallest and largest elements efficiently in BTreeSet
// - Use BTreeSet's ordered iteration for extremes
// - Practice iterator take() and rev() methods
// - Handle edge cases with k larger than set size
//
// Run with: cargo test --bin btree_k_extremes

/// Find the k smallest and k largest elements from a BTreeSet.
/// Return as two separate vectors in sorted order.
fn find_k_extremes<T>(
    set: &std::collections::BTreeSet<T>,
    k: usize,
) -> (Vec<T>, Vec<T>)
where
    T: Ord + Clone,
{
    todo!("Implement finding k smallest and largest elements")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_find_k_extremes() {
        let set: BTreeSet<i32> = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19].iter().cloned().collect();
        
        let (smallest, largest) = find_k_extremes(&set, 3);
        
        assert_eq!(smallest, vec![1, 3, 5]);
        assert_eq!(largest, vec![15, 17, 19]); // 3 largest in ascending order
        
        // Test with k larger than set size
        let (small_all, large_all) = find_k_extremes(&set, 20);
        assert_eq!(small_all.len(), set.len());
        assert_eq!(large_all.len(), set.len());
        
        // Test with k = 0
        let (empty_small, empty_large) = find_k_extremes(&set, 0);
        assert!(empty_small.is_empty());
        assert!(empty_large.is_empty());
    }

    #[test]
    fn test_edge_cases() {
        // Test with empty BTreeSet
        let empty_set: BTreeSet<i32> = BTreeSet::new();
        let (empty_small, empty_large) = find_k_extremes(&empty_set, 3);
        assert!(empty_small.is_empty());
        assert!(empty_large.is_empty());
    }
}