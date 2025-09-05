// HashMap Transformation Practice
//
// Learning objectives:
// - Transforming HashMap keys and values
// - Handling key collisions during transformation
// - Using custom transformation functions
//
// Run with: cargo test hashmap_transform

/// Transform HashMap keys and values using provided functions.
/// Handle potential key collisions by combining values.
pub fn transform_hashmap<K1, V1, K2, V2, FK, FV, FC>(
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
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        
        let transformed = transform_hashmap(
            map,
            |k| k * 10,    // Transform keys: 1->10, 2->20, 3->30
            |v| v.to_uppercase(), // Transform values to uppercase
            |v1, _v2| v1,  // No collisions expected, so combiner won't be used
        );
        
        assert_eq!(transformed.get(&10), Some(&"ONE".to_string()));
        assert_eq!(transformed.get(&20), Some(&"TWO".to_string()));
        assert_eq!(transformed.get(&30), Some(&"THREE".to_string()));
    }

    #[test]
    fn test_transform_with_key_collisions() {
        let mut map = HashMap::new();
        map.insert(1, 10);
        map.insert(11, 20);  // Both 1 and 11 will map to 1 after mod 10
        map.insert(21, 30);  // 21 will also map to 1 after mod 10
        
        let transformed = transform_hashmap(
            map,
            |k| k % 10,    // All keys become 1
            |v| v,         // Values unchanged
            |v1, v2| v1 + v2, // Sum colliding values
        );
        
        assert_eq!(transformed.get(&1), Some(&60)); // 10 + 20 + 30
        assert_eq!(transformed.len(), 1);
    }

    #[test]
    fn test_transform_empty_map() {
        let empty: HashMap<i32, String> = HashMap::new();
        let transformed = transform_hashmap(
            empty,
            |k| k + 1,
            |v| v.len(),
            |v1, v2| v1 + v2,
        );
        
        assert!(transformed.is_empty());
    }
}