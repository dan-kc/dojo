# Thread-Safe Counter Implementation

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadSafeCounter {
    count: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    pub fn new(initial: i32) -> Self {
        Self {
            count: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    pub fn get(&self) -> i32 {
        let count = self.count.lock().unwrap();
        *count
    }

    pub fn add(&self, value: i32) {
        let mut count = self.count.lock().unwrap();
        *count += value;
    }
}

// ThreadSafeCounter automatically implements Send and Sync because:
// - Arc<T> is Send + Sync when T: Send + Sync
// - Mutex<T> is Send + Sync when T: Send
// - i32 is Send + Sync

pub fn share_across_threads<T: Send + Sync + 'static>(value: T) -> Arc<T> {
    Arc::new(value)
}
```

## Explanation

This solution demonstrates implementing **Send + Sync traits** for thread-safe types:

### Key Concepts Demonstrated:

1. **Send and Sync Traits**:
   - **Send**: Type can be transferred between threads (ownership can be moved)
   - **Sync**: Type can be safely shared between threads (references can be shared)
   - `ThreadSafeCounter` is both Send and Sync due to its components

2. **Arc<Mutex<T>> Pattern**:
   - `Arc` provides shared ownership across multiple threads
   - `Mutex` provides mutual exclusion for safe mutation
   - This combination is the standard pattern for shared mutable state

3. **Automatic Trait Implementation**:
   - Rust automatically derives Send and Sync when components satisfy the requirements
   - `Arc<Mutex<i32>>` is Send + Sync because `i32` is Send + Sync and `Mutex` preserves these properties

4. **Thread-Safe Operations**:
   - All methods acquire the mutex lock before accessing the counter
   - Ensures atomic read/write operations
   - Prevents data races and ensures consistency

### How Send and Sync Work:

```rust
// Send: Can transfer ownership between threads
fn send_example<T: Send>(value: T) {
    thread::spawn(move || {
        // value is moved to this thread
        // This is safe because T: Send
    });
}

// Sync: Can share references between threads  
fn sync_example<T: Sync>(value: &T) {
    thread::spawn(move || {
        // Shared reference is used in this thread
        // This is safe because T: Sync
    });
}
```

### Thread Safety Guarantees:

1. **Data Race Prevention**: Mutex ensures only one thread can modify the counter at a time
2. **Memory Safety**: Arc ensures the counter lives as long as any thread needs it  
3. **Consistency**: All operations are atomic relative to each other

### Performance Considerations:

- **Lock Contention**: Multiple threads accessing the counter will serialize on the mutex
- **RAII**: Mutex guard automatically releases the lock when dropped
- **Poisoning**: Mutex becomes "poisoned" if a thread panics while holding the lock

### Alternative Implementations:

For high-performance scenarios, consider:
- **Atomic types**: `AtomicI32` for lock-free operations
- **Channels**: Message passing instead of shared state
- **RwLock**: Allows multiple concurrent readers

### Usage Pattern:

```rust
let counter = Arc::new(ThreadSafeCounter::new(0));
let counter_clone = counter.clone();

thread::spawn(move || {
    counter_clone.increment(); // Safe across threads
});

println!("Count: {}", counter.get()); // Thread-safe read
```

This implementation showcases Rust's type system ensuring thread safety at compile time while providing convenient shared mutable state through the Arc<Mutex<T>> pattern.