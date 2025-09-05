# Thread-Safe Cache - Solution

## Solution

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Implement a thread-safe shared cache using Arc<Mutex<T>>.
/// Multiple threads can safely read and write to the same cache.
pub struct ThreadSafeCache<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new thread-safe cache
    pub fn new() -> Self {
        ThreadSafeCache {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Insert a key-value pair
    pub fn insert(&self, key: K, value: V) {
        self.data.lock().unwrap().insert(key, value);
    }
    
    /// Get a value by key
    pub fn get(&self, key: &K) -> Option<V> {
        self.data.lock().unwrap().get(key).cloned()
    }
    
    /// Get or compute a value
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        let mut data = self.data.lock().unwrap();
        if let Some(value) = data.get(&key) {
            value.clone()
        } else {
            let value = compute();
            data.insert(key, value.clone());
            value
        }
    }
    
    /// Clone the cache handle (shares the same underlying data)
    pub fn clone_handle(&self) -> Self {
        ThreadSafeCache {
            data: Arc::clone(&self.data),
        }
    }
    
    /// Clear all entries
    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }
    
    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
}
```

## Explanation

### Arc<Mutex<T>> Pattern - Thread-Safe Shared State

**The Multi-Threading Challenge:**
```rust
// This doesn't work across threads - RefCell is not thread-safe
let cache = Rc::new(RefCell::new(HashMap::new()));
std::thread::spawn(move || {
    // ERROR: RefCell doesn't implement Send + Sync
    cache.borrow_mut().insert("key", "value");
});

// This works - Arc<Mutex<T>> provides thread-safe shared ownership
let cache = Arc::new(Mutex::new(HashMap::new()));
let cache_clone = Arc::clone(&cache);
std::thread::spawn(move || {
    cache_clone.lock().unwrap().insert("key", "value"); // ✓ Thread-safe
});
```

**Key Components:**
- **Arc<T>:** Atomic Reference Counting - thread-safe shared ownership
- **Mutex<T>:** Mutual Exclusion - thread-safe interior mutability
- **Combined:** Multiple threads can safely share and modify the same data

### Understanding Arc<T> vs Rc<T>

**Reference Counting Comparison:**
```rust
// Rc<T> - Single-threaded reference counting
Rc::new(data)    // Reference count stored in regular memory
Rc::clone(&rc)   // Fast increment (no atomic operations)

// Arc<T> - Atomic reference counting  
Arc::new(data)   // Reference count stored as atomic integer
Arc::clone(&arc) // Slower increment (atomic operations required)
```

**Memory Overhead:**
```rust
Rc<HashMap<K, V>>:  [strong: usize] + [weak: usize] + [HashMap<K, V>]
Arc<HashMap<K, V>>: [strong: AtomicUsize] + [weak: AtomicUsize] + [HashMap<K, V>]
//                   ^^^^^^^^^^^^^^^^^ More expensive operations
```

**Thread Safety:**
```rust
// Rc<T> traits
impl<T> !Send for Rc<T>  // Cannot be sent between threads
impl<T> !Sync for Rc<T>  // Cannot be shared between threads

// Arc<T> traits  
impl<T: Send + Sync> Send for Arc<T>  // Can be sent between threads
impl<T: Send + Sync> Sync for Arc<T>  // Can be shared between threads
```

### Understanding Mutex<T> vs RefCell<T>

**Locking Mechanisms:**
```rust
// RefCell<T> - Runtime borrow checking (single-threaded)
let cell = RefCell::new(data);
let borrowed = cell.borrow_mut();  // Panic if conflict

// Mutex<T> - OS-level locking (multi-threaded)
let mutex = Mutex::new(data);  
let guard = mutex.lock().unwrap(); // Blocks until available
```

**Performance Characteristics:**
| Aspect | RefCell<T> | Mutex<T> |
|--------|------------|----------|
| Thread Safety | No | Yes |
| Overhead | ~10-20 cycles | ~100-1000 cycles |
| Blocking | Panic on conflict | Block until available |
| Contention Handling | Immediate failure | Queue and wait |

### Locking Strategies and Patterns

**Basic Lock Acquisition:**
```rust
pub fn insert(&self, key: K, value: V) {
    let mut data = self.data.lock().unwrap();
    //     ^^^^ MutexGuard<HashMap<K, V>>
    data.insert(key, value);
} // Lock automatically released when guard drops
```

**Lock Duration Optimization:**
```rust
// BAD: Long-lived locks increase contention
pub fn bad_get_or_insert<F>(&self, key: K, compute: F) -> V 
where F: FnOnce() -> V 
{
    let mut data = self.data.lock().unwrap();
    if let Some(value) = data.get(&key) {
        value.clone()
    } else {
        let value = compute(); // Compute while holding lock!
        data.insert(key, value.clone());
        value
    }
}

// GOOD: Minimize lock duration
pub fn good_get_or_insert<F>(&self, key: K, compute: F) -> V 
where F: FnOnce() -> V 
{
    // Quick check with read-only lock
    {
        let data = self.data.lock().unwrap();
        if let Some(value) = data.get(&key) {
            return value.clone();
        }
    } // Lock released here
    
    // Compute without holding lock
    let value = compute();
    
    // Brief lock for insertion
    self.data.lock().unwrap().insert(key, value.clone());
    value
}
```

### Advanced Thread-Safe Patterns

**Read-Write Lock for Better Performance:**
```rust
use std::sync::RwLock;

pub struct OptimizedCache<K, V> {
    data: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> OptimizedCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn get(&self, key: &K) -> Option<V> {
        // Multiple readers can access simultaneously
        self.data.read().unwrap().get(key).cloned()
    }
    
    pub fn insert(&self, key: K, value: V) {
        // Exclusive writer access
        self.data.write().unwrap().insert(key, value);
    }
}
```

**Lock-Free Atomic Operations for Simple Data:**
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct AtomicCounter {
    count: Arc<AtomicUsize>,
}

impl AtomicCounter {
    pub fn new() -> Self {
        AtomicCounter {
            count: Arc::new(AtomicUsize::new(0)),
        }
    }
    
    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::Relaxed)
    }
    
    pub fn get(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
    
    pub fn clone_handle(&self) -> Self {
        AtomicCounter {
            count: Arc::clone(&self.count),
        }
    }
}
```

### Error Handling in Concurrent Code

**Mutex Poisoning:**
```rust
// Mutex becomes "poisoned" if a thread panics while holding the lock
pub fn robust_insert(&self, key: K, value: V) -> Result<(), &'static str> {
    match self.data.lock() {
        Ok(mut data) => {
            data.insert(key, value);
            Ok(())
        }
        Err(poisoned) => {
            // Mutex is poisoned - previous thread panicked
            // We can recover the data but should be cautious
            let mut data = poisoned.into_inner();
            data.clear(); // Clear potentially corrupted state
            data.insert(key, value);
            Ok(())
        }
    }
}
```

**Timeout-Based Locking:**
```rust
use std::time::Duration;

pub fn try_insert_with_timeout(&self, key: K, value: V, timeout: Duration) -> Result<(), &'static str> {
    match self.data.try_lock_for(timeout) {
        Ok(mut data) => {
            data.insert(key, value);
            Ok(())
        }
        Err(_) => Err("Could not acquire lock within timeout"),
    }
}
```

### Performance Optimization Techniques

**Batch Operations:**
```rust
impl<K, V> ThreadSafeCache<K, V> {
    /// Insert multiple items with a single lock acquisition
    pub fn insert_batch(&self, items: impl IntoIterator<Item = (K, V)>) {
        let mut data = self.data.lock().unwrap();
        for (key, value) in items {
            data.insert(key, value);
        }
        // Single lock for entire batch
    }
    
    /// Get multiple items efficiently
    pub fn get_batch(&self, keys: &[K]) -> Vec<Option<V>>
    where K: Clone
    {
        let data = self.data.lock().unwrap();
        keys.iter()
            .map(|key| data.get(key).cloned())
            .collect()
    }
}
```

**Pre-sizing and Capacity Management:**
```rust
impl<K, V> ThreadSafeCache<K, V> {
    /// Create cache with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        ThreadSafeCache {
            data: Arc::new(Mutex::new(HashMap::with_capacity(capacity))),
        }
    }
    
    /// Reserve additional capacity
    pub fn reserve(&self, additional: usize) {
        self.data.lock().unwrap().reserve(additional);
    }
}
```

### Deadlock Prevention

**Lock Ordering:**
```rust
// DANGEROUS: Potential deadlock if threads acquire locks in different orders
fn transfer_between_caches(cache1: &ThreadSafeCache<K, V>, cache2: &ThreadSafeCache<K, V>) {
    let _lock1 = cache1.data.lock().unwrap(); // Thread A gets this first
    let _lock2 = cache2.data.lock().unwrap(); // Thread B might have this first
    // Deadlock if both threads have opposite lock acquisition order!
}

// SAFE: Always acquire locks in consistent order
fn safe_transfer(cache1: &ThreadSafeCache<K, V>, cache2: &ThreadSafeCache<K, V>) {
    use std::ptr;
    
    let (first, second) = if ptr::addr_of!(*cache1.data) < ptr::addr_of!(*cache2.data) {
        (&cache1.data, &cache2.data)
    } else {
        (&cache2.data, &cache1.data)
    };
    
    let _lock1 = first.lock().unwrap();
    let _lock2 = second.lock().unwrap();
    // Always acquire in same order based on memory address
}
```

**Timeout-Based Deadlock Detection:**
```rust
use std::time::Duration;

fn try_double_lock(
    cache1: &ThreadSafeCache<K, V>, 
    cache2: &ThreadSafeCache<K, V>
) -> Result<(), &'static str> {
    let _lock1 = cache1.data.try_lock_for(Duration::from_millis(100))
        .map_err(|_| "Could not acquire first lock")?;
    
    let _lock2 = cache2.data.try_lock_for(Duration::from_millis(100))
        .map_err(|_| "Could not acquire second lock")?;
    
    // Both locks acquired successfully
    Ok(())
}
```

### Testing Concurrent Code

**Multi-threaded Testing:**
```rust
#[test]
fn test_concurrent_access() {
    let cache = Arc::new(ThreadSafeCache::new());
    let mut handles = Vec::new();
    
    // Spawn 10 threads that each insert 100 items
    for thread_id in 0..10 {
        let cache_clone = Arc::clone(&cache);
        let handle = std::thread::spawn(move || {
            for i in 0..100 {
                let key = format!("thread_{}_item_{}", thread_id, i);
                cache_clone.insert(key, thread_id * 100 + i);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all items were inserted
    assert_eq!(cache.len(), 1000);
}
```

**Race Condition Testing:**
```rust
#[test]
fn test_get_or_insert_race_condition() {
    let cache = Arc::new(ThreadSafeCache::new());
    let computation_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    // Multiple threads try to compute the same expensive value
    for _ in 0..10 {
        let cache_clone = Arc::clone(&cache);
        let count_clone = Arc::clone(&computation_count);
        
        let handle = std::thread::spawn(move || {
            cache_clone.get_or_insert_with("expensive_key".to_string(), || {
                count_clone.fetch_add(1, Ordering::Relaxed);
                std::thread::sleep(Duration::from_millis(10)); // Simulate work
                "expensive_result".to_string()
            })
        });
        handles.push(handle);
    }
    
    // Wait for completion
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Expensive computation should only happen once
    assert_eq!(computation_count.load(Ordering::Relaxed), 1);
    assert_eq!(cache.get(&"expensive_key".to_string()), Some("expensive_result".to_string()));
}
```

### Real-World Applications

**Web Server Connection Pool:**
```rust
use std::sync::Arc;

struct ConnectionPool {
    connections: ThreadSafeCache<String, DatabaseConnection>,
}

impl ConnectionPool {
    fn get_connection(&self, database_url: &str) -> DatabaseConnection {
        self.connections.get_or_insert_with(database_url.to_string(), || {
            // Expensive: Create new database connection
            DatabaseConnection::new(database_url)
        })
    }
}

// Multiple request handlers can share the same pool safely
let pool = Arc::new(ConnectionPool::new());
for _ in 0..num_threads {
    let pool_clone = Arc::clone(&pool);
    std::thread::spawn(move || {
        let conn = pool_clone.get_connection("postgresql://...");
        handle_requests(conn);
    });
}
```

**Distributed Computing Task Cache:**
```rust
struct TaskResultCache {
    results: ThreadSafeCache<TaskId, TaskResult>,
    in_progress: ThreadSafeCache<TaskId, Arc<Mutex<()>>>,
}

impl TaskResultCache {
    fn get_or_compute(&self, task_id: TaskId) -> TaskResult {
        // Check if result already exists
        if let Some(result) = self.results.get(&task_id) {
            return result;
        }
        
        // Ensure only one thread computes each task
        let lock = self.in_progress.get_or_insert_with(task_id.clone(), || {
            Arc::new(Mutex::new(()))
        });
        
        let _guard = lock.lock().unwrap();
        
        // Double-check after acquiring lock
        if let Some(result) = self.results.get(&task_id) {
            return result;
        }
        
        // Compute result
        let result = expensive_computation(task_id.clone());
        self.results.insert(task_id, result.clone());
        result
    }
}
```

### Best Practices

**Design Guidelines:**
1. **Minimize lock scope:** Hold locks for shortest time possible
2. **Consistent lock ordering:** Always acquire multiple locks in same order
3. **Use appropriate synchronization:** RwLock for read-heavy, Atomic for simple counters
4. **Handle lock poisoning:** Decide on recovery strategy for panicked threads
5. **Consider lock-free alternatives:** For high-performance scenarios

**Performance Tips:**
- **Batch operations** to reduce lock overhead
- **Pre-allocate capacity** to reduce memory allocations under lock
- **Use parking_lot** crate for potentially faster mutex implementations
- **Profile lock contention** to identify bottlenecks

**Common Anti-patterns:**
```rust
// BAD: Computing while holding lock
let mut data = cache.lock().unwrap();
let result = expensive_computation(); // Blocks other threads!
data.insert(key, result);

// BAD: Nested locking without ordering
let _lock1 = cache1.lock().unwrap();
let _lock2 = cache2.lock().unwrap(); // Potential deadlock

// BAD: Ignoring lock poisoning
let data = cache.lock().unwrap(); // Could panic!
```

The Arc<Mutex<T>> pattern demonstrates how Rust enables safe concurrent programming by combining atomic reference counting with mutual exclusion, providing both memory safety and data race freedom at the cost of some performance overhead and potential for deadlocks.