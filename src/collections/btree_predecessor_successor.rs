// BTree Predecessor/Successor Practice
//
// Learning Objectives:
// - Implement efficient predecessor/successor queries in BTreeSet
// - Use range methods to find adjacent elements
// - Handle edge cases for min/max elements
// - Practice ordered navigation in BTree collections
//
// Run with: cargo test --bin btree_predecessor_successor

/// Implement efficient predecessor/successor queries in BTreeSet.
/// Find the largest element < target and smallest element > target.
fn find_predecessor_successor<T>(
    set: &std::collections::BTreeSet<T>,
    target: &T,
) -> (Option<T>, Option<T>)
where
    T: Ord + Clone,
{
    todo!("Implement predecessor/successor finding")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_find_predecessor_successor() {
        let set: BTreeSet<i32> = [1, 3, 5, 7, 9, 11].iter().cloned().collect();
        
        // Target in set
        let (pred, succ) = find_predecessor_successor(&set, &5);
        assert_eq!(pred, Some(3));
        assert_eq!(succ, Some(7));
        
        // Target not in set
        let (pred, succ) = find_predecessor_successor(&set, &6);
        assert_eq!(pred, Some(5));
        assert_eq!(succ, Some(7));
        
        // Target smaller than all elements
        let (pred, succ) = find_predecessor_successor(&set, &0);
        assert_eq!(pred, None);
        assert_eq!(succ, Some(1));
        
        // Target larger than all elements
        let (pred, succ) = find_predecessor_successor(&set, &20);
        assert_eq!(pred, Some(11));
        assert_eq!(succ, None);
    }

    #[test]
    fn test_edge_cases() {
        // Test predecessor/successor with empty set
        let empty_set: BTreeSet<i32> = BTreeSet::new();
        let (no_pred, no_succ) = find_predecessor_successor(&empty_set, &5);
        assert_eq!(no_pred, None);
        assert_eq!(no_succ, None);
    }
}