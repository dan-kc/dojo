// Limited Cache Implementation Practice
//
// Learning Objectives:
// - Implement a cache with size limit using HashMap
// - Handle capacity management and eviction policies (FIFO)
// - Work with multiple data structures for complex behavior
// - Practice ownership and borrowing with mutable data structures
//
// Run with: cargo test --bin limited_cache

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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_cache_with_capacity_zero() {
        let mut cache = LimitedCache::new(0);
        assert_eq!(cache.insert("key", "value"), None);
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&"key"), None);
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = LimitedCache::new(3);
        
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);
        
        assert_eq!(cache.remove(&"b"), Some(2));
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.get(&"b"), None);
        
        // Can insert new item without evicting
        cache.insert("d", 4);
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"a"), Some(&1)); // Still present
    }

    #[test]
    fn test_cache_replacement_order() {
        let mut cache = LimitedCache::new(2);
        
        cache.insert("first", 1);
        cache.insert("second", 2);
        
        // Replace first item
        cache.insert("first", 10);
        
        // Add third item - should evict second (now oldest)
        cache.insert("third", 3);
        
        assert_eq!(cache.get(&"first"), Some(&10));
        assert_eq!(cache.get(&"second"), None); // Evicted
        assert_eq!(cache.get(&"third"), Some(&3));
    }
}