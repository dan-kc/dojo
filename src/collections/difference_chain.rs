// Run with: cargo test difference_chain

/// Implement set difference chain - consecutive differences between sets.
/// Given sets [A, B, C, D], return [A-B, B-C, C-D].
pub fn difference_chain<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut res = vec![];
    for sets in sets.windows(2) {
        let new_set: collections::HashSet<T> = sets[0].difference(&sets[1]).cloned().collect();

        res.push(new_set);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_difference_chain() {
        let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4, 5].iter().cloned().collect();
        let set3: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
        let set4: HashSet<i32> = [4, 5, 6, 7].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2, set3, set4]);

        assert_eq!(chain.len(), 3);

        // set1 - set2 = {1}
        let expected1: HashSet<i32> = [1].iter().cloned().collect();
        assert_eq!(chain[0], expected1);

        // set2 - set3 = {2}
        let expected2: HashSet<i32> = [2].iter().cloned().collect();
        assert_eq!(chain[1], expected2);

        // set3 - set4 = {3}
        let expected3: HashSet<i32> = [3].iter().cloned().collect();
        assert_eq!(chain[2], expected3);
    }

    #[test]
    fn test_difference_chain_empty_input() {
        let chain = difference_chain(vec![] as Vec<HashSet<i32>>);
        assert!(chain.is_empty());
    }

    #[test]
    fn test_difference_chain_single_set() {
        let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let chain = difference_chain(vec![set]);

        assert!(chain.is_empty()); // No differences to compute
    }

    #[test]
    fn test_difference_chain_two_sets() {
        let set1: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();
        let set2: HashSet<char> = ['b', 'c', 'd'].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2]);

        assert_eq!(chain.len(), 1);

        // set1 - set2 = {'a'}
        let expected: HashSet<char> = ['a'].iter().cloned().collect();
        assert_eq!(chain[0], expected);
    }

    #[test]
    fn test_difference_chain_identical_sets() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set3: HashSet<i32> = [1, 2, 3].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2, set3]);

        assert_eq!(chain.len(), 2);

        // Both differences should be empty
        assert!(chain[0].is_empty());
        assert!(chain[1].is_empty());
    }

    #[test]
    fn test_difference_chain_disjoint_sets() {
        let set1: HashSet<i32> = [1, 2].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4].iter().cloned().collect();
        let set3: HashSet<i32> = [5, 6].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2, set3]);

        assert_eq!(chain.len(), 2);

        // set1 - set2 = {1, 2} (no overlap)
        let expected1: HashSet<i32> = [1, 2].iter().cloned().collect();
        assert_eq!(chain[0], expected1);

        // set2 - set3 = {3, 4} (no overlap)
        let expected2: HashSet<i32> = [3, 4].iter().cloned().collect();
        assert_eq!(chain[1], expected2);
    }

    #[test]
    fn test_difference_chain_complete_overlap() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect(); // superset
        let set3: HashSet<i32> = [1, 2, 3, 4, 5, 6].iter().cloned().collect(); // superset

        let chain = difference_chain(vec![set1, set2, set3]);

        assert_eq!(chain.len(), 2);

        // set1 - set2 = {} (set1 is subset of set2)
        assert!(chain[0].is_empty());

        // set2 - set3 = {} (set2 is subset of set3)
        assert!(chain[1].is_empty());
    }

    #[test]
    fn test_difference_chain_partial_overlap() {
        let set1: HashSet<&str> = ["apple", "banana", "cherry"].iter().cloned().collect();
        let set2: HashSet<&str> = ["banana", "cherry", "date"].iter().cloned().collect();
        let set3: HashSet<&str> = ["cherry", "date", "elderberry"].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2, set3]);

        assert_eq!(chain.len(), 2);

        // set1 - set2 = {"apple"}
        let expected1: HashSet<&str> = ["apple"].iter().cloned().collect();
        assert_eq!(chain[0], expected1);

        // set2 - set3 = {"banana"}
        let expected2: HashSet<&str> = ["banana"].iter().cloned().collect();
        assert_eq!(chain[1], expected2);
    }

    #[test]
    fn test_difference_chain_empty_sets() {
        let empty1: HashSet<i32> = HashSet::new();
        let empty2: HashSet<i32> = HashSet::new();
        let set3: HashSet<i32> = [1, 2, 3].iter().cloned().collect();

        let chain = difference_chain(vec![empty1, empty2, set3]);

        assert_eq!(chain.len(), 2);

        // empty - empty = empty
        assert!(chain[0].is_empty());

        // empty - set3 = empty
        assert!(chain[1].is_empty());
    }

    #[test]
    fn test_difference_chain_single_elements() {
        let set1: HashSet<i32> = [1].iter().cloned().collect();
        let set2: HashSet<i32> = [2].iter().cloned().collect();
        let set3: HashSet<i32> = [3].iter().cloned().collect();
        let set4: HashSet<i32> = [4].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2, set3, set4]);

        assert_eq!(chain.len(), 3);

        // Each difference should be the original set (no overlap)
        let expected1: HashSet<i32> = [1].iter().cloned().collect();
        let expected2: HashSet<i32> = [2].iter().cloned().collect();
        let expected3: HashSet<i32> = [3].iter().cloned().collect();

        assert_eq!(chain[0], expected1);
        assert_eq!(chain[1], expected2);
        assert_eq!(chain[2], expected3);
    }

    #[test]
    fn test_difference_chain_large_sets() {
        let set1: HashSet<i32> = (1..=100).collect();
        let set2: HashSet<i32> = (50..=150).collect();
        let set3: HashSet<i32> = (100..=200).collect();

        let chain = difference_chain(vec![set1, set2, set3]);

        assert_eq!(chain.len(), 2);

        // set1 - set2 = {1..=49}
        let expected1: HashSet<i32> = (1..=49).collect();
        assert_eq!(chain[0], expected1);

        // set2 - set3 = {50..=99}
        let expected2: HashSet<i32> = (50..=99).collect();
        assert_eq!(chain[1], expected2);
    }

    #[test]
    fn test_difference_chain_strings() {
        let set1: HashSet<String> = ["hello", "world", "rust"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let set2: HashSet<String> = ["world", "rust", "programming"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let set3: HashSet<String> = ["rust", "programming", "language"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let chain = difference_chain(vec![set1, set2, set3]);

        assert_eq!(chain.len(), 2);

        // set1 - set2 = {"hello"}
        let expected1: HashSet<String> = ["hello"].iter().map(|s| s.to_string()).collect();
        assert_eq!(chain[0], expected1);

        // set2 - set3 = {"world"}
        let expected2: HashSet<String> = ["world"].iter().map(|s| s.to_string()).collect();
        assert_eq!(chain[1], expected2);
    }

    #[test]
    fn test_difference_chain_decreasing_size() {
        let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
        let set3: HashSet<i32> = [3].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2, set3]);

        assert_eq!(chain.len(), 2);

        // set1 - set2 = {1, 5}
        let expected1: HashSet<i32> = [1, 5].iter().cloned().collect();
        assert_eq!(chain[0], expected1);

        // set2 - set3 = {2, 4}
        let expected2: HashSet<i32> = [2, 4].iter().cloned().collect();
        assert_eq!(chain[1], expected2);
    }

    #[test]
    fn test_difference_chain_complex_pattern() {
        // Create sets with a complex overlapping pattern
        let set1: HashSet<i32> = [1, 2, 3, 4, 5, 6].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 4, 6, 8, 10].iter().cloned().collect();
        let set3: HashSet<i32> = [1, 3, 5, 7, 9].iter().cloned().collect();
        let set4: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();

        let chain = difference_chain(vec![set1, set2, set3, set4]);

        assert_eq!(chain.len(), 3);

        // set1 - set2 = {1, 3, 5}
        let expected1: HashSet<i32> = [1, 3, 5].iter().cloned().collect();
        assert_eq!(chain[0], expected1);

        // set2 - set3 = {2, 4, 6, 8, 10}
        let expected2: HashSet<i32> = [2, 4, 6, 8, 10].iter().cloned().collect();
        assert_eq!(chain[1], expected2);

        // set3 - set4 = {7, 9}
        let expected3: HashSet<i32> = [7, 9].iter().cloned().collect();
        assert_eq!(chain[2], expected3);
    }
}
