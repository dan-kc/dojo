// HashMap Operations Practice
//
// Learning Objectives:
// - Master HashMap creation, insertion, and lookup operations
// - Use the Entry API (or_insert, and_modify, or_default)
// - Practice key-value iteration and transformation
// - Work with HashMap merging and combining strategies
// - Understand custom key types and hashing
// - Compare performance characteristics with other collections
//
// Run with: cargo test --bin hashmap_operations

/// Use the Entry API to count character frequencies in a string.
/// Implement efficient counting using or_insert and and_modify.
fn count_char_frequencies(text: &str) -> std::collections::HashMap<char, usize> {
    todo!("Implement character frequency counting using Entry API")
}

/// Merge multiple hashmaps by combining their values using a provided function.
/// Use the Entry API to handle conflicts efficiently.
fn merge_hashmaps<K, V, F>(
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

/// Group vector elements by a key function, returning a HashMap of groups.
/// Use Entry API for efficient grouping.
fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    todo!("Implement grouping using HashMap")
}

/// Implement a cache with size limit using HashMap.
/// When capacity is exceeded, remove the oldest entry (FIFO).
struct LimitedCache<K, V> {
    map: std::collections::HashMap<K, V>,
    insertion_order: Vec<K>,
    capacity: usize,
}

impl<K, V> LimitedCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
{
    fn new(capacity: usize) -> Self {
        todo!("Implement new cache with capacity")
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!("Implement insert with capacity management")
    }

    fn get(&self, key: &K) -> Option<&V> {
        todo!("Implement get operation")
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        todo!("Implement remove operation")
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// Custom key type that demonstrates HashMap usage with complex keys.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CompoundKey {
    category: String,
    id: u32,
    flags: Vec<bool>,
}

impl CompoundKey {
    fn new(category: String, id: u32, flags: Vec<bool>) -> Self {
        Self { category, id, flags }
    }
}

/// Create and manipulate a HashMap with compound keys.
/// Demonstrate grouping and querying with complex key structures.
fn compound_key_operations() -> std::collections::HashMap<CompoundKey, String> {
    todo!("Implement operations with compound keys")
}

/// Implement efficient HashMap intersection that preserves values from the first map
/// where keys exist in both maps.
fn intersect_hashmaps<K, V>(
    map1: std::collections::HashMap<K, V>,
    map2: &std::collections::HashMap<K, V>,
) -> std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    todo!("Implement HashMap intersection")
}

/// Use HashMap to implement a simple word index for text search.
/// Map words to sets of positions where they appear.
fn build_word_index(text: &str) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
    todo!("Implement word position indexing")
}

/// Implement HashMap-based memoization for expensive function calls.
/// Use interior mutability for caching within an immutable context.
struct Memoizer<K, V> {
    cache: std::cell::RefCell<std::collections::HashMap<K, V>>,
}

impl<K, V> Memoizer<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        todo!("Implement new memoizer")
    }

    fn compute<F>(&self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        todo!("Implement memoized computation")
    }

    fn clear_cache(&self) {
        todo!("Implement cache clearing")
    }

    fn cache_size(&self) -> usize {
        todo!("Implement cache size reporting")
    }
}

/// Performance comparison function that measures HashMap vs Vec lookup times.
/// Use for educational purposes to understand when to choose each collection.
fn performance_comparison(data_size: usize) -> (std::time::Duration, std::time::Duration) {
    todo!("Implement HashMap vs Vec performance comparison")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_count_char_frequencies() {
        let text = "hello world";
        let freq = count_char_frequencies(text);
        
        assert_eq!(freq.get(&'l'), Some(&3));
        assert_eq!(freq.get(&'o'), Some(&2));
        assert_eq!(freq.get(&'h'), Some(&1));
        assert_eq!(freq.get(&'w'), Some(&1));
        assert_eq!(freq.get(&' '), Some(&1));
        assert_eq!(freq.get(&'z'), None);
        
        // Test empty string
        let empty_freq = count_char_frequencies("");
        assert!(empty_freq.is_empty());
    }

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
    fn test_group_by() {
        let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
        let grouped = group_by(words, |word| word.chars().next().unwrap());
        
        assert_eq!(grouped.get(&'a'), Some(&vec!["apple", "apricot"]));
        assert_eq!(grouped.get(&'b'), Some(&vec!["banana", "blueberry"]));
        assert_eq!(grouped.get(&'c'), Some(&vec!["cherry"]));
    }

    #[test]
    fn test_limited_cache() {
        let mut cache = LimitedCache::new(2);
        
        assert_eq!(cache.insert("a", 1), None);
        assert_eq!(cache.insert("b", 2), None);
        assert_eq!(cache.len(), 2);
        
        // Should evict "a" (oldest)
        assert_eq!(cache.insert("c", 3), None);
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
        
        // Replace existing key
        assert_eq!(cache.insert("b", 20), Some(2));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_compound_key_operations() {
        let map = compound_key_operations();
        
        // Test should create map with various compound keys
        assert!(!map.is_empty());
        
        // Test that we can query with compound keys
        let key1 = CompoundKey::new("category1".to_string(), 1, vec![true, false]);
        let key2 = CompoundKey::new("category1".to_string(), 2, vec![false, true]);
        
        // At least some keys should exist in the test data
        assert!(map.contains_key(&key1) || map.contains_key(&key2) || map.len() > 0);
    }

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
    fn test_build_word_index() {
        let text = "the quick brown fox jumps over the lazy dog";
        let index = build_word_index(text);
        
        let the_positions = index.get("the").unwrap();
        assert!(the_positions.contains(&0)); // First "the"
        assert!(the_positions.contains(&6)); // Second "the"
        assert_eq!(the_positions.len(), 2);
        
        let fox_positions = index.get("fox").unwrap();
        assert!(fox_positions.contains(&3));
        assert_eq!(fox_positions.len(), 1);
        
        assert_eq!(index.get("nonexistent"), None);
    }

    #[test]
    fn test_memoizer() {
        let memoizer = Memoizer::new();
        let mut call_count = std::cell::RefCell::new(0);
        
        let expensive_fn = |x: &i32| {
            *call_count.borrow_mut() += 1;
            x * x
        };
        
        // First call should compute
        let result1 = memoizer.compute(5, expensive_fn);
        assert_eq!(result1, 25);
        assert_eq!(*call_count.borrow(), 1);
        
        // Second call should use cache
        let result2 = memoizer.compute(5, expensive_fn);
        assert_eq!(result2, 25);
        assert_eq!(*call_count.borrow(), 1); // No additional call
        
        // Different key should compute
        let result3 = memoizer.compute(3, expensive_fn);
        assert_eq!(result3, 9);
        assert_eq!(*call_count.borrow(), 2);
        
        assert_eq!(memoizer.cache_size(), 2);
        
        memoizer.clear_cache();
        assert_eq!(memoizer.cache_size(), 0);
    }

    #[test]
    fn test_performance_comparison() {
        let (hashmap_time, vec_time) = performance_comparison(1000);
        
        // Both should complete in reasonable time
        assert!(hashmap_time < std::time::Duration::from_secs(1));
        assert!(vec_time < std::time::Duration::from_secs(1));
        
        // For large datasets and random access, HashMap should generally be faster
        // But this test is more for educational purposes
        println!("HashMap lookup time: {:?}", hashmap_time);
        println!("Vec lookup time: {:?}", vec_time);
    }

    #[test]
    fn test_edge_cases() {
        // Test empty hashmap operations
        let empty: HashMap<String, i32> = HashMap::new();
        let merged = merge_hashmaps(vec![empty], |a, b| a + b);
        assert!(merged.is_empty());
        
        // Test single element operations
        let freq = count_char_frequencies("a");
        assert_eq!(freq.len(), 1);
        assert_eq!(freq.get(&'a'), Some(&1));
        
        // Test cache with capacity 0
        let mut cache = LimitedCache::new(0);
        assert_eq!(cache.insert("key", "value"), None);
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&"key"), None);
    }

    #[test]
    fn test_compound_key_equality() {
        let key1 = CompoundKey::new("test".to_string(), 1, vec![true, false]);
        let key2 = CompoundKey::new("test".to_string(), 1, vec![true, false]);
        let key3 = CompoundKey::new("test".to_string(), 1, vec![false, true]);
        
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
        
        let mut map = HashMap::new();
        map.insert(key1, "value1".to_string());
        
        assert_eq!(map.get(&key2), Some(&"value1".to_string()));
        assert_eq!(map.get(&key3), None);
    }
}