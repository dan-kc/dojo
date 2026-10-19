# collections/memoizer

Reference implementation. Read after attempting the exercise.

```rust
pub struct Memoizer<K, V> {
    cache: std::cell::RefCell<std::collections::HashMap<K, V>>,
}

impl<K, V> Memoizer<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }

    pub fn compute<F>(&self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        if let Some(cached_value) = self.cache.borrow().get(&key) {
            return cached_value.clone();
        }

        let result = compute_fn(&key);

        self.cache.borrow_mut().insert(key, result.clone());

        result
    }

    pub fn clear_cache(&self) {
        self.cache.borrow_mut().clear();
    }

    pub fn cache_size(&self) -> usize {
        self.cache.borrow().len()
    }
}
```
