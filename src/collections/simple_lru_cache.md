# Simple LRU Cache Solution

## Implementation

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
            // Move key to end (most recently used)
            self.move_to_end(key);
            self.map.get(key)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Update existing key
            self.map.insert(key.clone(), value);
            self.move_to_end(&key);
        } else {
            // Check if we need to evict
            if self.map.len() >= self.capacity && self.capacity > 0 {
                self.evict_lru();
            }
            
            // Insert new key-value pair
            if self.capacity > 0 {
                self.map.insert(key.clone(), value);
                self.access_order.push(key);
            }
        }
    }

    fn move_to_end(&mut self, key: &K) {
        // Find and remove key from current position
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

## Explanation

This solution implements a simple LRU cache using HashMap and Vec:

1. **Storage structure**: HashMap for key-value storage, Vec for access order tracking
2. **Access tracking**: Vec maintains order of key access (LRU at front, MRU at back)
3. **Get operation**: Move accessed key to end of access order
4. **Put operation**: Handle existing keys and eviction when at capacity
5. **Eviction policy**: Remove least recently used key when capacity exceeded

The implementation prioritizes simplicity over optimal performance.

## Key Learning Points

- **LRU cache pattern**: Combination of fast lookup and access order tracking
- **Access order maintenance**: Moving elements to reflect recent usage
- **Capacity management**: Evicting old entries when limits reached
- **Cache semantics**: Understanding get/put behavior in caching systems

## Rust Concepts Demonstrated

- HashMap for key-value storage
- Vec for ordered data management
- Generic programming with trait bounds
- Clone trait for key management
- Cache eviction algorithms