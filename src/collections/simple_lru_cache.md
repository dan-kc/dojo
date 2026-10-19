# collections/simple_lru_cache

Reference implementation. Read after attempting the exercise.

```rust
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
        SimpleLRUCache {
            capacity,
            map: std::collections::HashMap::new(),
            access_order: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.move_to_end(key);
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.move_to_end(&key);
        } else {
            if self.map.len() >= self.capacity && self.capacity > 0 {
                self.evict_lru();
            }

            if self.capacity > 0 {
                self.map.insert(key.clone(), value);
                self.access_order.push(key);
            }
        }
    }

    fn move_to_end(&mut self, key: &K) {
        if let Some(pos) = self.access_order.iter().position(|k| k == key) {
            let key = self.access_order.remove(pos);
            self.access_order.push(key);
        }
    }

    fn evict_lru(&mut self) {
        if let Some(lru_key) = self.access_order.first().cloned() {
            self.access_order.remove(0);
            self.map.remove(&lru_key);
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
```
