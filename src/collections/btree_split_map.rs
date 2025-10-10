// cargo test btree_split_map

/// Split a BTreeMap at a given key, returning two maps: one with keys < split_key,
/// one with keys >= split_key. Original map should be consumed.

#[allow(dead_code)]
fn split_btree_map<K, V>(
    #[allow(unused_variables, unused_mut)] mut map: std::collections::BTreeMap<K, V>,
    #[allow(unused_variables)] split_key: &K,
) -> (
    std::collections::BTreeMap<K, V>,
    std::collections::BTreeMap<K, V>,
)
where
    K: Ord + Clone,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_split_btree_map() {
        let mut map = BTreeMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        map.insert(4, "four");
        map.insert(5, "five");

        let (left, right) = split_btree_map(map, &3);

        // Left should contain keys < 3
        assert_eq!(left.len(), 2);
        assert_eq!(left.get(&1), Some(&"one"));
        assert_eq!(left.get(&2), Some(&"two"));

        // Right should contain keys >= 3
        assert_eq!(right.len(), 3);
        assert_eq!(right.get(&3), Some(&"three"));
        assert_eq!(right.get(&4), Some(&"four"));
        assert_eq!(right.get(&5), Some(&"five"));
    }

    #[test]
    fn test_edge_cases() {
        // Test splitting empty map
        let empty_map: BTreeMap<i32, &str> = BTreeMap::new();
        let (left, right) = split_btree_map(empty_map, &5);
        assert!(left.is_empty());
        assert!(right.is_empty());
    }
}
