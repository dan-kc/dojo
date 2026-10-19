# concurrency/thread_safe_cache

Reference implementation. Read after attempting the exercise.

```rust
pub struct ThreadSafeCache<K, V> {
    data: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<K, V>>>,
}
impl<K: Eq + std::hash::Hash, V: Clone> ThreadSafeCache<K, V> {
    pub fn new() -> Self {
        Self {
            data: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    pub fn clone_handle(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
    pub fn put(&self, key: K, value: V) {
        self.data.write().unwrap().insert(key, value);
    }
    pub fn get(&self, key: &K) -> Option<V> {
        self.data.read().unwrap().get(key).cloned()
    }
    pub fn size(&self) -> usize {
        self.data.read().unwrap().len()
    }
    pub fn clear(&self) {
        self.data.write().unwrap().clear();
    }
    pub fn get_or_insert_with<F: FnOnce() -> V>(&self, key: K, compute: F) -> V {
        if let Some(value) = self.get(&key) {
            return value;
        }
        self.data
            .write()
            .unwrap()
            .entry(key)
            .or_insert_with(compute)
            .clone()
    }
}
```
