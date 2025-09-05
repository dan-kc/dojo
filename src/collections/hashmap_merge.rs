// HashMap Merging Practice
//
// Learning objectives:
// - Using Entry API for conflict resolution
// - Merging multiple HashMaps with custom combiners
// - Understanding value combination strategies
//
// Run with: cargo test hashmap_merge

/// Merge multiple hashmaps by combining their values using a provided function.
/// Use the Entry API to handle conflicts efficiently.
pub fn merge_hashmaps<K, V, F>(
    maps: Vec<std::collections::HashMap<K, V>>,
    combine_fn: F,
) -> std::collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
    F: Fn(V, V) -> V,
{
    todo!("Implement hashmap merging with value combination")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_merge_hashmaps() {
        let mut map1 = HashMap::new();
        map1.insert("a", 1);
        map1.insert("b", 2);
        
        let mut map2 = HashMap::new();
        map2.insert("b", 3);
        map2.insert("c", 4);
        
        let mut map3 = HashMap::new();
        map3.insert("a", 5);
        map3.insert("d", 6);
        
        let merged = merge_hashmaps(vec![map1, map2, map3], |v1, v2| v1 + v2);
        
        assert_eq!(merged.get("a"), Some(&6)); // 1 + 5
        assert_eq!(merged.get("b"), Some(&5)); // 2 + 3
        assert_eq!(merged.get("c"), Some(&4));
        assert_eq!(merged.get("d"), Some(&6));
    }

    #[test]
    fn test_merge_empty_maps() {
        let empty: HashMap<String, i32> = HashMap::new();
        let merged = merge_hashmaps(vec![empty], |a, b| a + b);
        assert!(merged.is_empty());
    }

    #[test]
    fn test_merge_with_max_combiner() {
        let mut map1 = HashMap::new();
        map1.insert("temp", 20);
        map1.insert("humidity", 60);
        
        let mut map2 = HashMap::new();
        map2.insert("temp", 25);
        map2.insert("pressure", 1013);
        
        let merged = merge_hashmaps(vec![map1, map2], |v1, v2| v1.max(v2));
        
        assert_eq!(merged.get("temp"), Some(&25)); // max(20, 25)
        assert_eq!(merged.get("humidity"), Some(&60));
        assert_eq!(merged.get("pressure"), Some(&1013));
    }

    #[test]
    fn test_merge_strings() {
        let mut map1 = HashMap::new();
        map1.insert("greeting", "Hello".to_string());
        
        let mut map2 = HashMap::new();
        map2.insert("greeting", " World".to_string());
        map2.insert("farewell", "Goodbye".to_string());
        
        let merged = merge_hashmaps(vec![map1, map2], |v1, v2| format!("{}{}", v1, v2));
        
        assert_eq!(merged.get("greeting"), Some(&"Hello World".to_string()));
        assert_eq!(merged.get("farewell"), Some(&"Goodbye".to_string()));
    }
}