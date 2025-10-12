// cargo test merge_hashmaps

/// Merge multiple hashmaps by combining their values using a provided function.
#[allow(unused_variables, dead_code)]
fn merge_hashmaps<K, V, F>(
    maps: Vec<std::collections::HashMap<K, V>>,
    combine_fn: F,
) -> std::collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
    F: Fn(V, V) -> V,
{
    todo!()
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
    fn test_merge_single_map() {
        let mut map = HashMap::new();
        map.insert("key", "value");

        let merged = merge_hashmaps(vec![map], |a, _b| a);
        assert_eq!(merged.get("key"), Some(&"value"));
    }

    #[test]
    fn test_merge_with_string_concatenation() {
        let mut map1 = HashMap::new();
        map1.insert("greeting", "Hello".to_string());

        let mut map2 = HashMap::new();
        map2.insert("greeting", " World".to_string());

        let merged = merge_hashmaps(vec![map1, map2], |a, b| format!("{}{}", a, b));
        assert_eq!(merged.get("greeting"), Some(&"Hello World".to_string()));
    }

    #[test]
    fn test_merge_with_max_function() {
        let mut map1 = HashMap::new();
        map1.insert("score", 85);
        map1.insert("level", 3);

        let mut map2 = HashMap::new();
        map2.insert("score", 92);
        map2.insert("attempts", 5);

        let merged = merge_hashmaps(vec![map1, map2], |a, b| a.max(b));

        assert_eq!(merged.get("score"), Some(&92)); // max(85, 92)
        assert_eq!(merged.get("level"), Some(&3));
        assert_eq!(merged.get("attempts"), Some(&5));
    }
}

