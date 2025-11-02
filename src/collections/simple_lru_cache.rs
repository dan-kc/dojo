// cargo test simple_lru_cache

/// Simple LRU cache implementation using HashMap and insertion order tracking.
/// This is a simplified version that doesn't use raw pointers.
pub struct SimpleLRUCache<K, V> {
    capacity: usize,
    map: std::collections::HashMap<K, V>,
    access_order: Vec<K>,
}

impl<K, V> SimpleLRUCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: std::collections::HashMap::new(),
            access_order: vec![],
        }
    }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        todo!()
    }

    pub fn put(&mut self, key: K, value: V) {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_lru_cache() {
        let mut cache = SimpleLRUCache::new(2);

        cache.put("key1", "value1");
        cache.put("key2", "value2");
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.capacity(), 2);

        assert_eq!(cache.get(&"key1"), Some(&"value1"));
        assert_eq!(cache.get(&"key2"), Some(&"value2"));

        // Adding third item should evict least recently used
        cache.put("key3", "value3");
        assert_eq!(cache.len(), 2);

        // Test that cache respects capacity
        assert!(cache.len() <= cache.capacity());
    }
}
