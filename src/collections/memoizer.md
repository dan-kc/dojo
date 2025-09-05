# Memoization Solution

## Implementation

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
        // Try to get from cache first
        if let Some(cached_value) = self.cache.borrow().get(&key) {
            return cached_value.clone();
        }
        
        // Compute the value
        let result = compute_fn(&key);
        
        // Cache the result
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

## Explanation

This solution implements function memoization using interior mutability:

1. **RefCell for interior mutability**: Allows mutable access to the cache through an immutable reference
2. **Cache-first lookup**: Checks if the result is already computed before calling the expensive function
3. **Lazy computation**: Only computes values when they're not already cached
4. **Generic implementation**: Works with any key-value types that meet the trait bounds

## Key Learning Points

- **Interior mutability**: Using RefCell to modify data through immutable references
- **Memoization pattern**: Caching expensive function results for reuse
- **Borrow checking**: `borrow()` and `borrow_mut()` provide runtime-checked access
- **Function parameters**: `FnOnce` ensures the compute function is called at most once per key

## Alternative Implementation with LRU Eviction

```rust
use std::collections::HashMap;
use std::cell::RefCell;

pub struct LRUMemoizer<K, V> {
    cache: RefCell<HashMap<K, V>>,
    usage_order: RefCell<Vec<K>>,
    capacity: usize,
}

impl<K, V> LRUMemoizer<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
            usage_order: RefCell::new(Vec::new()),
            capacity,
        }
    }

    pub fn compute<F>(&self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        // Check cache
        if let Some(cached_value) = self.cache.borrow().get(&key) {
            // Move to end (most recently used)
            let mut usage = self.usage_order.borrow_mut();
            if let Some(pos) = usage.iter().position(|k| k == &key) {
                usage.remove(pos);
            }
            usage.push(key);
            return cached_value.clone();
        }
        
        // Compute new value
        let result = compute_fn(&key);
        
        // Add to cache with LRU eviction
        let mut cache = self.cache.borrow_mut();
        let mut usage = self.usage_order.borrow_mut();
        
        if cache.len() >= self.capacity && !cache.contains_key(&key) {
            if let Some(oldest_key) = usage.remove(0) {
                cache.remove(&oldest_key);
            }
        }
        
        cache.insert(key.clone(), result.clone());
        usage.push(key);
        
        result
    }
}
```

## Use Cases

- **Expensive computations**: Fibonacci sequences, complex mathematical functions
- **Database queries**: Caching frequently accessed data
- **Web API responses**: Avoiding redundant network requests
- **File system operations**: Caching file metadata or content

## Rust Concepts Demonstrated

- Interior mutability patterns (RefCell)
- Runtime borrow checking and panic conditions
- Generic struct implementation with trait bounds
- Function parameters and closure types (FnOnce)
- Memory management and caching strategies
- Thread-unsafe shared mutable state patterns