// HashMap Intersection Practice
//
// Learning objectives:
// - Computing intersection of two HashMaps
// - Preserving values from specific maps during intersection
// - Using HashMap iteration and filtering
//
// Run with: cargo test intersect_hashmaps

/// Implement efficient HashMap intersection that preserves values from the first map
/// where keys exist in both maps.
pub fn intersect_hashmaps<K, V>(
    map1: std::collections::HashMap<K, V>,
    map2: &std::collections::HashMap<K, V>,
) -> std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    todo!("Implement HashMap intersection")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_intersect_hashmaps() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);
        map1.insert("c", 3);
        
        let mut map2 = HashMap::new();
        map2.insert("b", 20);
        map2.insert("c", 30);
        map2.insert("d", 40);
        
        let intersection = intersect_hashmaps(map1, &map2);
        
        assert_eq!(intersection.len(), 2);
        assert_eq!(intersection.get("b"), Some(&2)); // Value from map1
        assert_eq!(intersection.get("c"), Some(&3)); // Value from map1
        assert_eq!(intersection.get("a"), None);
        assert_eq!(intersection.get("d"), None);
    }

    #[test]
    fn test_intersect_no_common_keys() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);
        
        let mut map2 = HashMap::new();
        map2.insert("c", 3);
        map2.insert("d", 4);
        
        let intersection = intersect_hashmaps(map1, &map2);
        assert!(intersection.is_empty());
    }

    #[test]
    fn test_intersect_identical_keys() {
        let mut map1 = HashMap::new();
        map1.insert("x", 10);
        map1.insert("y", 20);
        
        let mut map2 = HashMap::new();
        map2.insert("x", 100);
        map2.insert("y", 200);
        
        let intersection = intersect_hashmaps(map1, &map2);
        
        assert_eq!(intersection.len(), 2);
        assert_eq!(intersection.get("x"), Some(&10)); // Values from map1
        assert_eq!(intersection.get("y"), Some(&20)); // Values from map1
    }

    #[test]
    fn test_intersect_empty_maps() {
        let empty1: HashMap<String, i32> = HashMap::new();
        let empty2: HashMap<String, i32> = HashMap::new();
        
        let intersection = intersect_hashmaps(empty1, &empty2);
        assert!(intersection.is_empty());
        
        let mut non_empty = HashMap::new();
        non_empty.insert("key".to_string(), 42);
        let empty: HashMap<String, i32> = HashMap::new();
        
        let intersection = intersect_hashmaps(non_empty, &empty);
        assert!(intersection.is_empty());
    }

    #[test]
    fn test_intersect_single_common_key() {
        let mut map1 = HashMap::new();
        map1.insert("common", 1);
        map1.insert("unique1", 2);
        map1.insert("unique2", 3);
        
        let mut map2 = HashMap::new();
        map2.insert("common", 10);
        map2.insert("unique3", 4);
        
        let intersection = intersect_hashmaps(map1, &map2);
        
        assert_eq!(intersection.len(), 1);
        assert_eq!(intersection.get("common"), Some(&1)); // Value from map1
    }
}