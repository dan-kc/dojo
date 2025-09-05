# Limited Cache Implementation - Solution

## Solution

```rust
impl<K, V> LimitedCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
{
    fn new(capacity: usize) -> Self {
        Self {
            map: std::collections::HashMap::new(),
            insertion_order: Vec::new(),
            capacity,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Check if key already exists
        if let Some(old_value) = self.map.insert(key.clone(), value) {
            // Key existed - update insertion order by moving to end
            if let Some(pos) = self.insertion_order.iter().position(|k| k == &key) {
                self.insertion_order.remove(pos);
            }
            self.insertion_order.push(key);
            Some(old_value)
        } else {
            // New key - add to insertion order
            self.insertion_order.push(key);
            
            // Check if we need to evict
            if self.map.len() > self.capacity && self.capacity > 0 {
                // Remove oldest (first) entry
                if let Some(oldest_key) = self.insertion_order.first().cloned() {
                    self.map.remove(&oldest_key);
                    self.insertion_order.remove(0);
                }
            }
            None
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.remove(key) {
            // Remove from insertion order
            if let Some(pos) = self.insertion_order.iter().position(|k| k == key) {
                self.insertion_order.remove(pos);
            }
            Some(value)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
```

## Explanation

This implementation creates a FIFO (First-In-First-Out) cache with the following key features:

1. **Dual data structure approach**: 
   - `HashMap` for O(1) key-value operations
   - `Vec` to track insertion order for eviction

2. **Capacity management**: When the cache exceeds capacity, the oldest entry is removed.

3. **Key replacement handling**: When an existing key is updated, it's moved to the end of the insertion order.

4. **Efficient operations**:
   - `insert()`: O(1) average case, O(n) worst case due to Vec operations
   - `get()`: O(1) HashMap lookup
   - `remove()`: O(n) due to Vec linear search and removal

**Key Rust concepts demonstrated:**
- **Generic structs**: Works with any key-value types meeting trait bounds
- **Trait bounds**: `K` must be `Clone + Hash + Eq` for HashMap usage
- **Ownership management**: Careful handling of owned vs borrowed data
- **Option types**: Proper error handling for cache misses
- **Vector operations**: Using `position()` and `remove()` for order tracking

**Alternative implementations:**
- For better performance, consider using `VecDeque` or a doubly-linked list
- For LRU (Least Recently Used), update order on both reads and writes
- For more complex eviction policies, consider priority queues or timestamps

**Use cases:**
- HTTP response caching
- Database query result caching
- Memoization with memory limits
- Resource management in memory-constrained environments