// BTree Consecutive Ranges Practice
//
// Learning Objectives:
// - Find consecutive number ranges in ordered BTreeSet
// - Use ordered iteration to detect consecutive sequences
// - Practice generic constraints for arithmetic operations
// - Handle edge cases with single elements and gaps
//
// Run with: cargo test --bin btree_consecutive_ranges

/// Find all subranges in a BTreeSet where consecutive elements differ by exactly 1.
/// Return the ranges as (start, end) pairs.
fn find_consecutive_ranges<T>(
    set: &std::collections::BTreeSet<T>,
) -> Vec<(T, T)>
where
    T: Ord + Clone + std::ops::Add<Output = T> + From<u8> + PartialEq,
{
    todo!("Implement consecutive range finding")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_find_consecutive_ranges() {
        let set: BTreeSet<i32> = [1, 2, 3, 5, 6, 10, 11, 12, 13, 20].iter().cloned().collect();
        let ranges = find_consecutive_ranges(&set);
        
        // Expected ranges: [1,3], [5,6], [10,13], and [20,20]
        assert_eq!(ranges.len(), 4);
        assert!(ranges.contains(&(1, 3)));
        assert!(ranges.contains(&(5, 6)));
        assert!(ranges.contains(&(10, 13)));
        assert!(ranges.contains(&(20, 20)));
    }

    #[test]
    fn test_edge_cases() {
        // Test consecutive ranges with single elements
        let single_set: BTreeSet<i32> = [5].iter().cloned().collect();
        let single_ranges = find_consecutive_ranges(&single_set);
        assert_eq!(single_ranges, vec![(5, 5)]);
    }
}