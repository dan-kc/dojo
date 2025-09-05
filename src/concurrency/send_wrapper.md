# Send Wrapper for Thread Safety

## Solution

```rust
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

pub struct SendWrapper<T> {
    inner: Mutex<T>,
}

impl<T> SendWrapper<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let guard = self.inner.lock().unwrap();
        f(&*guard)
    }

    pub fn with_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.inner.lock().unwrap();
        f(&mut *guard)
    }
}

// SendWrapper<T> is automatically Send when T is Send
// This is because Mutex<T> is Send when T is Send
// The Mutex provides the synchronization needed for safe transfer between threads

// SendWrapper<T> is automatically Sync when T is Send  
// This is because Mutex<T> is Sync when T is Send
// The Mutex allows safe concurrent access from multiple threads
```

## Explanation

This solution demonstrates **wrapping non-Send types** to make them usable across threads:

### Key Concepts Demonstrated:

1. **Send Wrapper Pattern**:
   - Wraps potentially non-Send types in a `Mutex` to make them Send
   - Provides safe access methods that enforce synchronization
   - Allows types to be moved between threads safely

2. **Interior Mutability with Mutex**:
   - `Mutex<T>` provides thread-safe interior mutability
   - Allows mutation through shared references (`&self`)
   - Ensures exclusive access during mutation

3. **Automatic Trait Implementation**:
   - `SendWrapper<T>` automatically implements `Send` when `T: Send`
   - `SendWrapper<T>` automatically implements `Sync` when `T: Send` 
   - Rust's trait system handles the complex thread safety requirements

4. **Safe Access Patterns**:
   - `with()`: Provides immutable access to the wrapped value
   - `with_mut()`: Provides mutable access to the wrapped value
   - Both methods ensure the mutex is properly acquired and released

### How It Works:

```rust
// Before wrapping (might not be Send):
struct NotSend {
    data: std::rc::Rc<i32>, // Rc is not Send
}

// After wrapping:
let wrapper = SendWrapper::new(NotSend { 
    data: std::rc::Rc::new(42) 
});

// Can now be used across threads:
thread::spawn(move || {
    wrapper.with(|inner| {
        // Safe access to NotSend data
        println!("Value: {:?}", inner.data);
    });
});
```

### Send vs Sync Clarification:

- **Send**: Type can be **transferred** between threads (ownership moves)
- **Sync**: Type can be **shared** between threads (references can be shared)

`SendWrapper<T>` achieves both by:
- Using `Mutex` to serialize access (making sharing safe)
- Allowing the entire wrapper to be moved between threads

### When to Use SendWrapper:

1. **Legacy Types**: Wrapping types that aren't thread-safe but need to cross thread boundaries
2. **FFI Types**: C types that aren't automatically Send/Sync
3. **Reference Counted Types**: Like `Rc<T>` that need to be used across threads
4. **Complex State**: Types with interior pointers or non-thread-safe components

### Alternative Approaches:

- **Arc instead of Rc**: For reference-counted data, use `Arc<T>` directly
- **Channels**: Message passing instead of shared state
- **Atomic types**: For simple data that can use lock-free operations

### Performance Implications:

- **Lock overhead**: Every access requires mutex acquisition
- **Serialization**: All access is serialized through the mutex
- **Memory overhead**: Additional space for the mutex

### Safety Guarantees:

The wrapper ensures:
1. **Data races are prevented**: Mutex serializes access
2. **Memory safety**: No use-after-free or double-free issues
3. **Thread safety**: Safe to share across thread boundaries

This pattern demonstrates how Rust's type system can be leveraged to safely wrap non-thread-safe types, providing both convenience and safety guarantees.