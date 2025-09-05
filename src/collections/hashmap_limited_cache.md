# Limited Cache Solution

## Implementation

```rust
pub struct LimitedCache<K, V> {
    map: std::collections::HashMap<K, V>,
    insertion_order: Vec<K>,
    capacity: usize,
}

impl<K, V> LimitedCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            map: std::collections::HashMap::new(),
            insertion_order: Vec::new(),
            capacity,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.capacity == 0 {
            return None;
        }
        
        // If key already exists, update it without changing insertion order
        if let Some(old_value) = self.map.insert(key.clone(), value) {
            return Some(old_value);
        }
        
        // New key insertion
        self.insertion_order.push(key.clone());
        
        // Check if we need to evict
        if self.map.len() > self.capacity {
            if let Some(oldest_key) = self.insertion_order.remove(0) {
                self.map.remove(&oldest_key);
            }
        }
        
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.remove(key) {
            // Remove from insertion order tracking
            if let Some(pos) = self.insertion_order.iter().position(|k| k == key) {
                self.insertion_order.remove(pos);
            }
            Some(value)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
```

## Explanation

This solution implements a FIFO (First In, First Out) limited cache:

1. **Dual data structures**: HashMap for O(1) lookups, Vec for insertion order tracking
2. **Capacity enforcement**: Automatically evicts oldest entries when capacity is exceeded
3. **Update semantics**: Updating existing keys preserves their insertion order position
4. **Zero capacity handling**: Gracefully handles edge case of zero capacity

## Key Learning Points

- **FIFO eviction policy**: Removes the oldest inserted item when capacity is reached
- **Order tracking**: Using Vec to maintain insertion order alongside HashMap
- **Entry API usage**: `map.insert()` returns the old value if the key existed
- **Synchronization**: Keeping Vec and HashMap in sync during all operations

## Alternative Implementation (Using VecDeque)

```rust
use std::collections::{HashMap, VecDeque};

pub struct LimitedCache<K, V> {
    map: HashMap<K, V>,
    insertion_order: VecDeque<K>,
    capacity: usize,
}

impl<K, V> LimitedCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            insertion_order: VecDeque::new(),
            capacity,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.capacity == 0 {
            return None;
        }
        
        if let Some(old_value) = self.map.insert(key.clone(), value) {
            return Some(old_value);
        }
        
        self.insertion_order.push_back(key);
        
        if self.map.len() > self.capacity {
            if let Some(oldest_key) = self.insertion_order.pop_front() {
                self.map.remove(&oldest_key);
            }
        }
        
        None
    }
    
    // ... rest of the implementation
}
```

## Rust Concepts Demonstrated

- Struct composition with multiple collection types
- Generic implementation with trait bounds
- FIFO data structure patterns
- Memory management and capacity constraints
- Interior consistency between related data structures
- Edge case handling (zero capacity, empty cache)