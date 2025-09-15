// BTree Range Query Practice
//
// Learning Objectives:
// - Use BTreeMap range methods for efficient range queries
// - Practice inclusive range queries with key-value pairs
// - Understand ordered iteration in BTree collections
// - Master range() method and its variants
//
// Run with: cargo test btree_range_query

/// Implement range queries on a BTreeMap to find all entries within a key range.
/// Return both keys and values within the specified range (inclusive).
#[allow(dead_code)]
fn range_query<K, V>(
    #[allow(unused_variables)] map: &std::collections::BTreeMap<K, V>,
    #[allow(unused_variables)] start: &K,
    #[allow(unused_variables)] end: &K,
) -> Vec<(K, V)>
where
    K: Ord + Clone,
    V: Clone,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_range_query() {
        let mut map = BTreeMap::new();
        map.insert(1, "one");
        map.insert(3, "three");
        map.insert(5, "five");
        map.insert(7, "seven");
        map.insert(9, "nine");

        let result = range_query(&map, &3, &7);
        assert_eq!(result, vec![(3, "three"), (5, "five"), (7, "seven")]);

        // Test empty range
        let empty_result = range_query(&map, &2, &2);
        assert!(empty_result.is_empty());

        // Test range with no matches
        let no_matches = range_query(&map, &10, &20);
        assert!(no_matches.is_empty());
    }

    #[test]
    fn test_edge_cases() {
        // Test with empty BTreeMap
        let empty_map: BTreeMap<i32, &str> = BTreeMap::new();
        let empty_range = range_query(&empty_map, &1, &10);
        assert!(empty_range.is_empty());
    }
}
