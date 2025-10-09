// cargo test hashmap_limited_cache

/// Implement a cache with size limit using HashMap.
/// When capacity is exceeded, remove the oldest entry (FIFO).
#[allow(dead_code)]
pub struct LimitedCache<K, V> {
    map: std::collections::HashMap<K, V>,
    insertion_order: Vec<K>,
    capacity: usize,
}

impl<K, V> LimitedCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
{
    #[allow(unused_variables)]
    pub fn new(capacity: usize) -> Self {
        todo!("Implement new cache with capacity")
    }

    #[allow(unused_variables)]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        todo!("Implement insert with capacity management")
    }

    #[allow(unused_variables)]
    pub fn get(&self, key: &K) -> Option<&V> {
        todo!("Implement get operation")
    }

    #[allow(unused_variables)]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        todo!("Implement remove operation")
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
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
    fn test_cache_zero_capacity() {
        let mut cache = LimitedCache::new(0);
        assert_eq!(cache.insert("key", "value"), None);
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.get(&"key"), None);
    }

    #[test]
    fn test_cache_single_capacity() {
        let mut cache = LimitedCache::new(1);

        assert_eq!(cache.insert("first", 1), None);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"first"), Some(&1));

        // Should evict "first"
        assert_eq!(cache.insert("second", 2), None);
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"first"), None);
        assert_eq!(cache.get(&"second"), Some(&2));
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

        // Should be able to add without eviction now
        cache.insert("d", 4);
        assert_eq!(cache.len(), 3);
        assert!(cache.get(&"a").is_some());
        assert!(cache.get(&"c").is_some());
        assert!(cache.get(&"d").is_some());
    }

    #[test]
    fn test_cache_update_existing() {
        let mut cache = LimitedCache::new(2);

        cache.insert("a", 1);
        cache.insert("b", 2);

        // Update existing key should not affect insertion order
        assert_eq!(cache.insert("a", 10), Some(1));

        // "a" should still be considered older than "b"
        cache.insert("c", 3);
        assert_eq!(cache.get(&"a"), None); // "a" should be evicted
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }
}

