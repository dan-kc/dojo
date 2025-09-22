// BTree Stepped Union Practice
//
// Learning Objectives:
// - Implement iterative union operations with intermediate results
// - Practice building union incrementally across multiple sets
// - Use BTreeSet union operations while tracking progress
// - Return intermediate steps of set operations
//
// Run with: cargo test btree_stepped_union

/// Implement set operations that maintain order and return intermediate steps.
/// Return each step of the union operation as it builds the result.
#[allow(dead_code)]
fn stepped_union<T>(
    #[allow(unused_variables)] sets: Vec<std::collections::BTreeSet<T>>,
) -> Vec<std::collections::BTreeSet<T>>
where
    T: Ord + Clone,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_stepped_union() {
        let set1: BTreeSet<i32> = [1, 3].iter().cloned().collect();
        let set2: BTreeSet<i32> = [2, 4].iter().cloned().collect();
        let set3: BTreeSet<i32> = [3, 5].iter().cloned().collect();

        let steps = stepped_union(vec![set1, set2, set3]);

        assert_eq!(steps.len(), 3);

        // Step 0: just set1
        let expected_step0: BTreeSet<i32> = [1, 3].iter().cloned().collect();
        assert_eq!(steps[0], expected_step0);

        // Step 1: set1 ∪ set2
        let expected_step1: BTreeSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        assert_eq!(steps[1], expected_step1);

        // Step 2: set1 ∪ set2 ∪ set3
        let expected_step2: BTreeSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        assert_eq!(steps[2], expected_step2);
    }
}
