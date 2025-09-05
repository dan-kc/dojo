// BTree Set Operations Practice
//
// Learning Objectives:
// - Implement ordered set operations using BTreeSet
// - Practice union, intersection, and difference operations
// - Maintain ordering while performing set operations
// - Use BTreeSet methods for efficient set computations
//
// Run with: cargo test --bin btree_set_operations

/// Implement ordered set operations: ordered union, intersection, difference.
/// Return results as BTreeSets to maintain ordering.
fn ordered_set_operations<T>(
    set_a: &std::collections::BTreeSet<T>,
    set_b: &std::collections::BTreeSet<T>,
) -> (std::collections::BTreeSet<T>, std::collections::BTreeSet<T>, std::collections::BTreeSet<T>)
where
    T: Ord + Clone,
{
    todo!("Implement ordered union, intersection, and difference")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_ordered_set_operations() {
        let set_a: BTreeSet<i32> = [1, 3, 5, 7].iter().cloned().collect();
        let set_b: BTreeSet<i32> = [3, 5, 7, 9].iter().cloned().collect();
        
        let (union, intersection, difference) = ordered_set_operations(&set_a, &set_b);
        
        let expected_union: BTreeSet<i32> = [1, 3, 5, 7, 9].iter().cloned().collect();
        let expected_intersection: BTreeSet<i32> = [3, 5, 7].iter().cloned().collect();
        let expected_difference: BTreeSet<i32> = [1].iter().cloned().collect();
        
        assert_eq!(union, expected_union);
        assert_eq!(intersection, expected_intersection);
        assert_eq!(difference, expected_difference);
    }
}