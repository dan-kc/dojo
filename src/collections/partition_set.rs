// cargo test partition_set

/// Implement set partitioning based on a predicate function.
/// Return two sets: one with elements matching the predicate, one without.
#[allow(unused_variables)]
pub fn partition_set<T, F>(
    set: std::collections::HashSet<T>,
    predicate: F,
) -> (std::collections::HashSet<T>, std::collections::HashSet<T>)
where
    T: std::hash::Hash + Eq,
    F: Fn(&T) -> bool,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_partition_set() {
        let set: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().cloned().collect();
        let (evens, odds) = partition_set(set, |&x| x % 2 == 0);

        let expected_evens: HashSet<i32> = [2, 4, 6, 8, 10].iter().cloned().collect();
        let expected_odds: HashSet<i32> = [1, 3, 5, 7, 9].iter().cloned().collect();

        assert_eq!(evens, expected_evens);
        assert_eq!(odds, expected_odds);

        // Test with all elements matching predicate
        let all_evens: HashSet<i32> = [2, 4, 6].iter().cloned().collect();
        let (matching, non_matching) = partition_set(all_evens, |&x| x % 2 == 0);
        assert_eq!(matching.len(), 3);
        assert!(non_matching.is_empty());
    }

    #[test]
    fn test_partition_strings() {
        let set: HashSet<&str> = ["short", "a", "longer", "word", "x"]
            .iter()
            .cloned()
            .collect();
        let (long_words, short_words) = partition_set(set, |&s| s.len() > 3);

        let expected_long: HashSet<&str> = ["short", "longer", "word"].iter().cloned().collect();
        let expected_short: HashSet<&str> = ["a", "x"].iter().cloned().collect();

        assert_eq!(long_words, expected_long);
        assert_eq!(short_words, expected_short);
    }

    #[test]
    fn test_partition_empty_set() {
        let empty_set: HashSet<i32> = HashSet::new();
        let (matching, non_matching) = partition_set(empty_set, |&x| x > 0);

        assert!(matching.is_empty());
        assert!(non_matching.is_empty());
    }

    #[test]
    fn test_partition_all_match() {
        let set: HashSet<i32> = [2, 4, 6, 8].iter().cloned().collect();
        let (matching, non_matching) = partition_set(set.clone(), |&x| x % 2 == 0);

        assert_eq!(matching, set);
        assert!(non_matching.is_empty());
    }

    #[test]
    fn test_partition_none_match() {
        let set: HashSet<i32> = [1, 3, 5, 7].iter().cloned().collect();
        let (matching, non_matching) = partition_set(set.clone(), |&x| x % 2 == 0);

        assert!(matching.is_empty());
        assert_eq!(non_matching, set);
    }

    #[test]
    fn test_partition_complex_predicate() {
        let set: HashSet<i32> = [1, 4, 9, 16, 25, 36, 49, 64].iter().cloned().collect();
        // Partition perfect squares that are also even
        let (even_squares, rest) = partition_set(set, |&x| {
            let sqrt = (x as f64).sqrt() as i32;
            sqrt * sqrt == x && x % 2 == 0
        });

        let expected_even_squares: HashSet<i32> = [4, 16, 36, 64].iter().cloned().collect();
        let expected_rest: HashSet<i32> = [1, 9, 25, 49].iter().cloned().collect();

        assert_eq!(even_squares, expected_even_squares);
        assert_eq!(rest, expected_rest);
    }
}
