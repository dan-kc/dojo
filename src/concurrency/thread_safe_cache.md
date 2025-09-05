**Solution:**

```rust
struct ThreadSafeCache<K, V> {
    data: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<K, V>>>,
}

impl<K, V> ThreadSafeCache<K, V> 
where 
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn new() -> Self {
        ThreadSafeCache {
            data: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    fn put(&self, key: K, value: V) {
        let mut cache = self.data.write().unwrap();
        cache.insert(key, value);
    }

    fn get(&self, key: &K) -> Option<V> {
        let cache = self.data.read().unwrap();
        cache.get(key).cloned()
    }

    fn size(&self) -> usize {
        let cache = self.data.read().unwrap();
        cache.len()
    }
}
```

**Explanation:**

This solution demonstrates the use of RwLock for read-write synchronization. Key concepts:

1. **RwLock**: Allows multiple concurrent readers OR a single writer, but not both
2. **Read Lock**: Multiple threads can hold read locks simultaneously for better performance
3. **Write Lock**: Only one thread can hold a write lock, blocking all other access
4. **Performance Optimization**: RwLock is ideal when reads significantly outnumber writes

The RwLock provides better concurrency than Mutex for read-heavy workloads because multiple threads can read simultaneously. This pattern is common in caches, configuration stores, and any shared data structure with infrequent updates.