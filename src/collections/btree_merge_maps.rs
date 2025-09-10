// BTree Map Merging Practice
//
// Learning Objectives:
// - Merge multiple sorted BTreeMaps efficiently
// - Use combining functions to handle value conflicts
// - Practice consuming multiple collections
// - Understand ordered merging strategies
//
// Run with: cargo test btree_merge_maps

/// Merge multiple sorted BTreeMaps, combining values for duplicate keys.
/// Use a combining function to handle value conflicts.
fn merge_sorted_btreemaps<K, V, F>(
    maps: Vec<std::collections::BTreeMap<K, V>>,
    combine_fn: F,
) -> std::collections::BTreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
    F: Fn(V, V) -> V,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_merge_sorted_btreemaps() {
        let mut map1 = BTreeMap::new();
        map1.insert("a", 1);
        map1.insert("c", 3);

        let mut map2 = BTreeMap::new();
        map2.insert("b", 2);
        map2.insert("c", 5); // Conflict with map1

        let mut map3 = BTreeMap::new();
        map3.insert("d", 4);
        map3.insert("c", 7); // Another conflict

        let merged = merge_sorted_btreemaps(vec![map1, map2, map3], |v1, v2| v1 + v2);

        assert_eq!(merged.get("a"), Some(&1));
        assert_eq!(merged.get("b"), Some(&2));
        assert_eq!(merged.get("c"), Some(&15)); // 3 + 5 + 7
        assert_eq!(merged.get("d"), Some(&4));

        // Verify ordering is maintained
        let keys: Vec<_> = merged.keys().collect();
        assert_eq!(keys, vec![&"a", &"b", &"c", &"d"]);
    }
}
