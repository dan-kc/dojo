// Mutable Cache Practice
//
// Learning objectives:
// - Using RefCell for interior mutability in data structures
// - Implementing cache patterns with lazy evaluation
// - Understanding borrow checker implications
//
// Run with: cargo test mutable_cache

use std::cell::RefCell;

/// Implement a cache with interior mutability that can be used through immutable references.
pub struct MutableCache<K, V> {
    cache: RefCell<std::collections::HashMap<K, V>>,
}

impl<K, V> MutableCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new empty cache
    pub fn new() -> Self {
        todo!("Initialize with empty HashMap in RefCell")
    }
    
    /// Get or compute a value for the given key
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        todo!("Check if key exists, if not compute and insert, then return value")
    }
    
    /// Insert a value for the given key
    pub fn insert(&self, key: K, value: V) {
        todo!("Borrow mutably and insert key-value pair")
    }
    
    /// Check if the cache contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        todo!("Borrow immutably and check if key exists")
    }
    
    /// Clear all entries from the cache
    pub fn clear(&self) {
        todo!("Borrow mutably and clear HashMap")
    }
    
    /// Get the number of cached entries
    pub fn len(&self) -> usize {
        todo!("Borrow immutably and return HashMap length")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic_operations() {
        let cache = MutableCache::new();
        
        // Cache should start empty
        assert_eq!(cache.len(), 0);
        assert!(!cache.contains_key(&"key1".to_string()));
        
        // Insert and check
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.len(), 1);
        assert!(cache.contains_key(&"key1".to_string()));
    }

    #[test]
    fn test_cache_get_or_insert_with() {
        let cache = MutableCache::new();
        let mut call_count = 0;
        
        // First call should compute the value
        let result1 = cache.get_or_insert_with("expensive".to_string(), || {
            call_count += 1;
            "computed_value".to_string()
        });
        assert_eq!(result1, "computed_value");
        assert_eq!(call_count, 1);
        
        // Second call should use cached value
        let result2 = cache.get_or_insert_with("expensive".to_string(), || {
            call_count += 1;
            "should_not_be_called".to_string()
        });
        assert_eq!(result2, "computed_value");
        assert_eq!(call_count, 1); // Computation function wasn't called again
    }

    #[test]
    fn test_cache_clear() {
        let cache = MutableCache::new();
        cache.insert("key1".to_string(), 1);
        cache.insert("key2".to_string(), 2);
        
        assert_eq!(cache.len(), 2);
        
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(!cache.contains_key(&"key1".to_string()));
        assert!(!cache.contains_key(&"key2".to_string()));
    }
}