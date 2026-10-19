// cargo test --lib concurrency::thread_safe_cache::

//! Provide a shared cache with concurrent reads and exclusive writes. Clone handles share the same data.
//! put replaces values; get clones them. get_or_insert_with must compute a missing key exactly once even with simultaneous callers.
//! Computations run under exclusive access and must not re-enter this cache. Poisoning may panic.
//! clear removes all entries; size reports distinct keys.

pub struct ThreadSafeCache<K, V> {
    data: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<K, V>>>,
}
impl<K: Eq + std::hash::Hash, V: Clone> ThreadSafeCache<K, V> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn clone_handle(&self) -> Self {
        todo!()
    }
    pub fn put(&self, key: K, value: V) {
        todo!()
    }
    pub fn get(&self, key: &K) -> Option<V> {
        todo!()
    }
    pub fn size(&self) -> usize {
        todo!()
    }
    pub fn clear(&self) {
        todo!()
    }
    pub fn get_or_insert_with<F: FnOnce() -> V>(&self, key: K, compute: F) -> V {
        todo!()
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
        let handles: Vec<_> = (0..5)
            .map(|i| {
                let cache = cache_clone.clone();
                thread::spawn(move || {
                    cache.put(format!("key{}", i), format!("value{}", i));
                    cache.get(&format!("key{}", i))
                })
            })
            .collect();

        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.join().unwrap();
            assert_eq!(result, Some(format!("value{i}")));
        }

        assert_eq!(cache.size(), 5);
    }
}
#[cfg(test)]
mod miss_tests {
    use super::*;
    #[test]
    fn concurrent_miss_computes_once() {
        let cache = ThreadSafeCache::new();
        let calls = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|s| {
            let mut handles = vec![];
            for _ in 0..8 {
                let c = cache.clone_handle();
                let calls = &calls;
                handles.push(s.spawn(move || {
                    c.get_or_insert_with("key", || {
                        calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        42
                    })
                }));
            }
            for h in handles {
                assert_eq!(h.join().unwrap(), 42);
            }
        });
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), 1);
        cache.clear();
        assert_eq!(cache.size(), 0);
    }
}
