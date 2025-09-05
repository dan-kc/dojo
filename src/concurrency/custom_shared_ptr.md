# Custom Shared Pointer Implementation

## Solution

```rust
use std::sync::Arc;

pub struct CustomSharedPtr<T> {
    inner: Arc<T>,
}

impl<T> CustomSharedPtr<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }

    pub fn get(&self) -> Arc<T> {
        Arc::clone(&self.inner)
    }

    pub fn clone_ptr(&self) -> CustomSharedPtr<T> {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

// Send and Sync implementation:
// CustomSharedPtr<T> is automatically Send when T: Send + Sync
// CustomSharedPtr<T> is automatically Sync when T: Send + Sync
// This is because Arc<T> is Send + Sync when T: Send + Sync

unsafe impl<T: Send + Sync> Send for CustomSharedPtr<T> {}
unsafe impl<T: Send + Sync> Sync for CustomSharedPtr<T> {}
```

## Explanation

This solution demonstrates **custom smart pointer implementation** with proper Send + Sync trait bounds:

### Key Concepts Demonstrated:

1. **Smart Pointer Wrapper**:
   - Wraps `Arc<T>` to provide custom behavior
   - Maintains reference counting semantics  
   - Adds application-specific methods or constraints

2. **Reference Counting Semantics**:
   - `Arc::clone()` increments reference count (cheap operation)
   - Shared ownership across multiple threads
   - Automatic cleanup when last reference is dropped

3. **Explicit Send + Sync Implementation**:
   - `unsafe impl` explicitly states thread safety requirements
   - Requires `T: Send + Sync` for safe sharing
   - Documents the safety contract for users

4. **Safe API Design**:
   - `get()` returns a new `Arc<T>` handle to the same data
   - `clone_ptr()` creates a new pointer to the same data
   - Both operations are safe and maintain reference counting

### How Arc Works:

```rust
// Reference counting example:
let ptr1 = CustomSharedPtr::new(42);     // ref_count = 1
let ptr2 = ptr1.clone_ptr();             // ref_count = 2
let arc_handle = ptr1.get();             // ref_count = 3

// When ptr1, ptr2, and arc_handle go out of scope:
// ref_count decreases: 3 -> 2 -> 1 -> 0 -> data is freed
```

### Send + Sync Requirements:

For `CustomSharedPtr<T>` to be `Send + Sync`, `T` must be:
- **Send**: Can be transferred between threads
- **Sync**: Can be safely accessed concurrently from multiple threads

```rust
// Safe usage:
let shared_data = CustomSharedPtr::new(vec![1, 2, 3, 4]);

thread::spawn(move || {
    // Can move CustomSharedPtr between threads (Send)
    let data_ref = shared_data.get();
    println!("Length: {}", data_ref.len());
});

// Can share references across threads (Sync)
let shared_ref = &shared_data;
thread::spawn(move || {
    let clone = shared_ref.clone_ptr(); // Safe because Sync
});
```

### Why Explicit Send + Sync Implementation:

While Rust would normally derive these automatically, explicit implementation:
1. **Documents Intent**: Makes thread safety requirements clear
2. **Custom Constraints**: Allows adding additional safety requirements
3. **API Stability**: Ensures consistent behavior across Rust versions
4. **Educational**: Shows how trait bounds propagate

### Comparison with Standard Arc:

| Feature | `Arc<T>` | `CustomSharedPtr<T>` |
|---------|----------|---------------------|
| Reference Counting | ✓ | ✓ (via Arc) |
| Thread Safety | ✓ | ✓ |
| Custom Methods | ✗ | ✓ (extensible) |
| Zero-cost | ✓ | Small overhead |

### Use Cases for Custom Shared Pointers:

1. **Resource Management**: Adding custom cleanup logic
2. **Debugging**: Tracking allocation/deallocation
3. **Metrics**: Counting references or access patterns
4. **Type Safety**: Restricting operations on shared data
5. **API Abstraction**: Hiding Arc complexity from users

### Safety Considerations:

The `unsafe impl` is safe because:
- `Arc<T>` is already `Send + Sync` when `T: Send + Sync`
- Our wrapper doesn't add any unsafe operations
- The trait bounds ensure `T` meets the requirements

### Memory Management:

```rust
// Automatic cleanup when all references are dropped:
{
    let ptr1 = CustomSharedPtr::new(expensive_data());
    let ptr2 = ptr1.clone_ptr();
    
    // Both ptr1 and ptr2 share the same data
    // Reference count = 2
} 
// When scope ends, reference count goes to 0
// expensive_data() is automatically freed
```

### Alternative Implementations:

For different use cases:
- **Weak references**: Add `Weak<T>` support to break cycles
- **Custom allocators**: Use custom memory allocation strategies  
- **Copy-on-write**: Implement COW semantics for mutable sharing

This pattern demonstrates how to create safe, thread-safe smart pointers in Rust while maintaining the performance and safety guarantees of the underlying `Arc<T>` implementation.