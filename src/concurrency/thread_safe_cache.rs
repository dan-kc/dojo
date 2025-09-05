// Thread-Safe Cache with RwLock Practice
//
// Learning Objectives:
// - Use RwLock for reader-writer scenarios
// - Allow concurrent reads but exclusive writes
// - Implement thread-safe data structures
//
// cargo test --bin thread_safe_cache

/// Create a shared cache that multiple threads can read from and write to.
/// Implement get, put, and size operations that are thread-safe.
/// Use RwLock to allow concurrent reads but exclusive writes.
struct ThreadSafeCache<K, V> {
    data: std::marker::PhantomData<(K, V)>, // Define your fields here
}

impl<K, V> ThreadSafeCache<K, V> 
where 
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn new() -> Self {
        todo!("Implement new")
    }

    fn put(&self, key: K, value: V) {
        todo!("Implement put")
    }

    fn get(&self, key: &K) -> Option<V> {
        todo!("Implement get")
    }

    fn size(&self) -> usize {
        todo!("Implement size")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_thread_safe_cache() {
        let cache = Arc::new(ThreadSafeCache::new());
        let cache_clone = cache.clone();
        
        // Test basic operations
        cache.put("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(cache.get(&"nonexistent".to_string()), None);
        assert_eq!(cache.size(), 1);
        
        // Test concurrent access
        let handles: Vec<_> = (0..5).map(|i| {
            let cache = cache_clone.clone();
            thread::spawn(move || {
                cache.put(format!("key{}", i), format!("value{}", i));
                cache.get(&format!("key{}", i))
            })
        }).collect();
        
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_some());
        }
        
        assert!(cache.size() >= 5);
    }
}