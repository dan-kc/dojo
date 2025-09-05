# Mutable Cache - Solution

## Solution

```rust
use std::cell::RefCell;

/// Implement a cache with interior mutability that can be used through immutable references.
pub struct MutableCache<K, V> {
    cache: RefCell<std::collections::HashMap<K, V>>,
}

impl<K, V> MutableCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new empty cache
    pub fn new() -> Self {
        MutableCache {
            cache: RefCell::new(std::collections::HashMap::new()),
        }
    }
    
    /// Get or compute a value for the given key
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        // First, try to get the value with an immutable borrow
        {
            let cache = self.cache.borrow();
            if let Some(value) = cache.get(&key) {
                return value.clone();
            }
        } // Immutable borrow released here
        
        // Value not found, compute it
        let value = compute();
        
        // Insert the computed value with a mutable borrow
        self.cache.borrow_mut().insert(key, value.clone());
        value
    }
    
    /// Insert a value for the given key
    pub fn insert(&self, key: K, value: V) {
        self.cache.borrow_mut().insert(key, value);
    }
    
    /// Check if the cache contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.borrow().contains_key(key)
    }
    
    /// Clear all entries from the cache
    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }
    
    /// Get the number of cached entries
    pub fn len(&self) -> usize {
        self.cache.borrow().len()
    }
}
```

## Explanation

### Interior Mutability for Caching Patterns

**The Caching Problem:**
```rust
// Traditional mutable API - requires mutable reference
impl Cache<K, V> {
    fn get(&mut self, key: K) -> V {  // Requires &mut self
        // Problem: Can't share cache between multiple users
        // Problem: Caller must have mutable access
    }
}

// Interior mutability solution - works with immutable reference
impl MutableCache<K, V> {
    fn get_or_insert_with<F>(&self, key: K, compute: F) -> V {  // Only needs &self
        // Solution: Multiple users can share the same cache
        // Solution: Cache appears immutable but can update internally
    }
}
```

**Key Insight:** Caches are conceptually immutable from the outside (they provide data lookup) but need to mutate internally (to store computed values). RefCell<T> perfectly bridges this semantic gap.

### Two-Phase Borrow Strategy

**The Challenge:**
We need to check if a key exists, and if not, compute and insert the value. This requires transitioning from read to write access safely.

**Naive Approach (Problematic):**
```rust
// DON'T DO THIS - holds borrow too long
pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V {
    let mut cache = self.cache.borrow_mut();  // Mutable borrow for entire operation
    if let Some(value) = cache.get(&key) {
        value.clone()
    } else {
        let value = compute();  // What if compute() tries to use the cache?
        cache.insert(key, value.clone());
        value
    }
}
```

**Safe Two-Phase Approach:**
```rust
pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V {
    // Phase 1: Check if value exists (immutable borrow)
    {
        let cache = self.cache.borrow();
        if let Some(value) = cache.get(&key) {
            return value.clone();  // Early return with existing value
        }
    } // Immutable borrow released here
    
    // Phase 2: Compute and insert (allows compute() to use cache)
    let value = compute();  // No borrows active, safe for recursion
    self.cache.borrow_mut().insert(key, value.clone());
    value
}
```

### Cache Access Patterns

**Read-Heavy Operations:**
```rust
pub fn contains_key(&self, key: &K) -> bool {
    self.cache.borrow().contains_key(key)  // Short-lived immutable borrow
}

pub fn len(&self) -> usize {
    self.cache.borrow().len()  // Quick read operation
}
```

**Write Operations:**
```rust
pub fn insert(&self, key: K, value: V) {
    self.cache.borrow_mut().insert(key, value);  // Exclusive write access
}

pub fn clear(&self) {
    self.cache.borrow_mut().clear();  // Exclusive access for bulk operation
}
```

### Advanced Caching Patterns

**Recursive Caching:**
The two-phase approach enables recursive caching where `compute()` might itself call the cache:

```rust
fn fibonacci_cache(cache: &MutableCache<i32, i64>, n: i32) -> i64 {
    cache.get_or_insert_with(n, || {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci_cache(cache, n - 1) + fibonacci_cache(cache, n - 2)
            // ^ These recursive calls work because no borrow is active
        }
    })
}
```

**Batch Operations:**
```rust
impl<K, V> MutableCache<K, V> {
    pub fn get_batch(&self, keys: &[K]) -> Vec<Option<V>>
    where
        K: Clone,
        V: Clone,
    {
        let cache = self.cache.borrow();  // Single borrow for all reads
        keys.iter()
            .map(|key| cache.get(key).cloned())
            .collect()
    }
    
    pub fn insert_batch(&self, items: impl IntoIterator<Item = (K, V)>) {
        let mut cache = self.cache.borrow_mut();  // Single borrow for all writes
        for (key, value) in items {
            cache.insert(key, value);
        }
    }
}
```

### Memory Management and Performance

**Memory Layout:**
```rust
MutableCache<String, i32> {
    cache: RefCell<HashMap<String, i32>>
}

// Memory structure:
// [borrow_flag: isize] + [HashMap: ptr + len + capacity] + heap data
// RefCell overhead: ~8 bytes
// HashMap overhead: ~24 bytes + hash table + entries
```

**Performance Characteristics:**

| Operation | Time Complexity | Borrow Overhead |
|-----------|----------------|-----------------|
| `get_or_insert_with` (hit) | O(1) average | 1 immutable borrow |
| `get_or_insert_with` (miss) | O(1) + compute time | 1 immutable + 1 mutable |
| `contains_key` | O(1) average | 1 immutable borrow |
| `insert` | O(1) average | 1 mutable borrow |
| `clear` | O(n) | 1 mutable borrow |

### Thread Safety Considerations

**Single-Threaded Cache:**
```rust
// RefCell is NOT thread-safe
let cache = MutableCache::new();
// Can't share between threads - would need Arc<Mutex<HashMap<K, V>>>
```

**Thread-Safe Alternative:**
```rust
use std::sync::{Arc, Mutex};

pub struct ThreadSafeCache<K, V> {
    cache: Arc<Mutex<HashMap<K, V>>>,
}

// Usage with threads
let cache = Arc::new(ThreadSafeCache::new());
for i in 0..10 {
    let cache_clone = cache.clone();
    std::thread::spawn(move || {
        cache_clone.insert(i, i * i);
    });
}
```

### Error Handling Strategies

**Panic Prevention:**
```rust
impl<K, V> MutableCache<K, V> {
    /// Safe version that won't panic on borrow conflicts
    pub fn try_get(&self, key: &K) -> Result<Option<V>, &'static str>
    where
        K: std::hash::Hash + Eq,
        V: Clone,
    {
        match self.cache.try_borrow() {
            Ok(cache) => Ok(cache.get(key).cloned()),
            Err(_) => Err("Cache is currently being modified"),
        }
    }
    
    /// Safe insertion that handles borrow conflicts
    pub fn try_insert(&self, key: K, value: V) -> Result<(), &'static str> {
        match self.cache.try_borrow_mut() {
            Ok(mut cache) => {
                cache.insert(key, value);
                Ok(())
            }
            Err(_) => Err("Cache is currently being accessed"),
        }
    }
}
```

### Lazy Evaluation Patterns

**Memoization:**
```rust
use std::cell::RefCell;

struct LazyComputer {
    cache: MutableCache<String, i32>,
}

impl LazyComputer {
    fn expensive_computation(&self, input: &str) -> i32 {
        self.cache.get_or_insert_with(input.to_string(), || {
            // Simulate expensive computation
            std::thread::sleep(std::time::Duration::from_millis(100));
            input.len() as i32 * 42
        })
    }
}

// First call: computes and caches
let result1 = computer.expensive_computation("hello");  // Takes 100ms

// Subsequent calls: return cached value instantly
let result2 = computer.expensive_computation("hello");  // Instant
```

**Configuration Caching:**
```rust
struct ConfigCache {
    cache: MutableCache<String, String>,
}

impl ConfigCache {
    fn get_config(&self, key: &str) -> String {
        self.cache.get_or_insert_with(key.to_string(), || {
            // Load from file, environment, or network
            std::env::var(key).unwrap_or_else(|_| "default".to_string())
        })
    }
}
```

### Testing Caching Behavior

**Cache Hit/Miss Testing:**
```rust
#[test]
fn test_cache_behavior() {
    let cache = MutableCache::new();
    let mut call_count = 0;
    
    // First call should compute
    let result1 = cache.get_or_insert_with("test".to_string(), || {
        call_count += 1;
        42
    });
    assert_eq!(result1, 42);
    assert_eq!(call_count, 1);
    
    // Second call should use cached value
    let result2 = cache.get_or_insert_with("test".to_string(), || {
        call_count += 1;
        999  // Should not be called
    });
    assert_eq!(result2, 42);
    assert_eq!(call_count, 1);  // Compute function not called again
}
```

### Best Practices

**Design Principles:**
1. **Keep borrows minimal:** Release RefCell borrows as quickly as possible
2. **Use two-phase access:** Separate read and write phases to avoid conflicts
3. **Consider batch operations:** Group multiple operations under single borrow
4. **Plan for recursion:** Ensure compute functions can safely use the cache

**Common Pitfalls:**
```rust
// BAD: Long-lived borrow
let cache_ref = cache.cache.borrow();
do_complex_work(&cache_ref);  // Risk of panic if anything needs to mutate

// BAD: Storing borrow guards
struct BadCache {
    cached_ref: Ref<'_, HashMap<K, V>>,  // Lifetime nightmare!
}

// BAD: Ignoring borrow conflicts
cache.cache.borrow_mut().insert(key, value);  // Could panic

// GOOD: Short, scoped borrows
{
    let cache = cache.cache.borrow();
    let value = cache.get(&key);
}  // Borrow released immediately
```

### Real-World Applications

**Web Server Response Caching:**
```rust
struct ResponseCache {
    cache: MutableCache<String, Response>,
}

impl ResponseCache {
    fn get_response(&self, url: &str) -> Response {
        self.cache.get_or_insert_with(url.to_string(), || {
            // Expensive network request
            fetch_from_network(url)
        })
    }
}
```

**Database Query Caching:**
```rust
struct QueryCache {
    cache: MutableCache<String, QueryResult>,
}

impl QueryCache {
    fn execute_query(&self, sql: &str) -> QueryResult {
        self.cache.get_or_insert_with(sql.to_string(), || {
            database.execute(sql)
        })
    }
}
```

The MutableCache pattern demonstrates how RefCell enables elegant solutions to common programming problems by providing controlled interior mutability while maintaining a clean, immutable-appearing API.