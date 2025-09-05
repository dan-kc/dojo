// HashMap Transformation Practice
//
// Learning Objectives:
// - Transform HashMap keys and values using provided functions
// - Handle potential key collisions by combining values
// - Work with complex generic functions and trait bounds
// - Practice functional programming patterns with HashMap
//
// Run with: cargo test --bin transform_hashmap

/// Transform HashMap keys and values using provided functions.
/// Handle potential key collisions by combining values.
fn transform_hashmap<K1, V1, K2, V2, FK, FV, FC>(
    map: std::collections::HashMap<K1, V1>,
    key_fn: FK,
    value_fn: FV,
    combine_fn: FC,
) -> std::collections::HashMap<K2, V2>
where
    K2: std::hash::Hash + Eq,
    FK: Fn(K1) -> K2,
    FV: Fn(V1) -> V2,
    FC: Fn(V2, V2) -> V2,
{
    todo!("Implement key-value transformation with collision handling")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_transform_hashmap() {
        let mut map = HashMap::new();
        map.insert("one", 1);
        map.insert("two", 2);
        map.insert("ONE", 10); // Will collide with "one" after lowercase
        
        let transformed = transform_hashmap(
            map,
            |k: &str| k.to_lowercase(),
            |v| v * 2,
            |v1, v2| v1 + v2,
        );
        
        assert_eq!(transformed.get("one"), Some(&22)); // (1 * 2) + (10 * 2)
        assert_eq!(transformed.get("two"), Some(&4)); // 2 * 2
    }

    #[test]
    fn test_transform_no_collisions() {
        let mut map = HashMap::new();
        map.insert(1, "a");
        map.insert(2, "b");
        map.insert(3, "c");
        
        let transformed = transform_hashmap(
            map,
            |k| k * 10,           // Transform keys: 1->10, 2->20, 3->30
            |v| v.to_uppercase(), // Transform values: "a"->"A", etc.
            |v1, _v2| v1,         // No collisions expected
        );
        
        assert_eq!(transformed.get(&10), Some(&"A".to_string()));
        assert_eq!(transformed.get(&20), Some(&"B".to_string()));
        assert_eq!(transformed.get(&30), Some(&"C".to_string()));
    }

    #[test]
    fn test_transform_multiple_collisions() {
        let mut map = HashMap::new();
        map.insert("apple", 5);
        map.insert("apricot", 7);
        map.insert("banana", 6);
        map.insert("blueberry", 9);
        
        // Transform to first letter, summing lengths
        let transformed = transform_hashmap(
            map,
            |k: &str| k.chars().next().unwrap(), // First character
            |v| v,                               // Keep value as-is
            |v1, v2| v1 + v2,                   // Sum values for same first letter
        );
        
        assert_eq!(transformed.get(&'a'), Some(&12)); // apple(5) + apricot(7)
        assert_eq!(transformed.get(&'b'), Some(&15)); // banana(6) + blueberry(9)
    }

    #[test]
    fn test_transform_empty_map() {
        let empty: HashMap<i32, String> = HashMap::new();
        let transformed = transform_hashmap(
            empty,
            |k| k,
            |v| v,
            |v1, _v2| v1,
        );
        
        assert!(transformed.is_empty());
    }
}