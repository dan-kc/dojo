// Thread-Safe Cache Practice
//
// Learning objectives:
// - Combining Arc<T> with Mutex<T>
// - Thread-safe shared mutable data
// - Understanding performance implications of locking
//
// Run with: cargo test thread_safe_cache

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Implement a thread-safe shared cache using Arc<Mutex<T>>.
/// Multiple threads can safely read and write to the same cache.
pub struct ThreadSafeCache<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new thread-safe cache
    pub fn new() -> Self {
        todo!("Initialize with Arc<Mutex<HashMap>>")
    }
    
    /// Insert a key-value pair
    pub fn insert(&self, key: K, value: V) {
        todo!("Lock mutex and insert")
    }
    
    /// Get a value by key
    pub fn get(&self, key: &K) -> Option<V> {
        todo!("Lock mutex and get cloned value")
    }
    
    /// Get or compute a value
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        todo!("Lock once, check if exists, compute if needed, insert and return")
    }
    
    /// Clone the cache handle (shares the same underlying data)
    pub fn clone_handle(&self) -> Self {
        todo!("Clone the Arc")
    }
    
    /// Clear all entries
    pub fn clear(&self) {
        todo!("Lock mutex and clear HashMap")
    }
    
    /// Get the number of entries
    pub fn len(&self) -> usize {
        todo!("Lock mutex and return HashMap length")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_thread_safe_cache_basic() {
        let cache = ThreadSafeCache::new();
        
        cache.insert("key1".to_string(), 42);
        assert_eq!(cache.get(&"key1".to_string()), Some(42));
        assert_eq!(cache.len(), 1);
        
        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_thread_safe_cache_sharing() {
        let cache = ThreadSafeCache::new();
        let cache_clone = cache.clone_handle();
        
        cache.insert("shared".to_string(), 100);
        assert_eq!(cache_clone.get(&"shared".to_string()), Some(100));
        
        cache_clone.insert("from_clone".to_string(), 200);
        assert_eq!(cache.get(&"from_clone".to_string()), Some(200));
    }

    #[test]
    fn test_thread_safe_cache_get_or_insert() {
        let cache = ThreadSafeCache::new();
        
        let result1 = cache.get_or_insert_with("computed".to_string(), || 42);
        assert_eq!(result1, 42);
        
        let result2 = cache.get_or_insert_with("computed".to_string(), || 100);
        assert_eq!(result2, 42); // Should return cached value, not recompute
    }

    #[test]
    fn test_thread_safe_cache_multithreaded() {
        let cache = Arc::new(ThreadSafeCache::new());
        let mut handles = Vec::new();
        
        // Spawn multiple threads that insert values
        for i in 0..5 {
            let cache_clone = cache.clone();
            let handle = thread::spawn(move || {
                cache_clone.insert(format!("key_{}", i), i * 10);
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify all values were inserted
        assert_eq!(cache.len(), 5);
        for i in 0..5 {
            assert_eq!(cache.get(&format!("key_{}", i)), Some(i * 10));
        }
    }
}