# Send to Thread Utilities

## Solution

```rust
use std::thread;

pub fn send_to_thread<T: Send + 'static, F: FnOnce(T) -> T + Send + 'static>(
    value: T,
    processor: F,
) -> T {
    let handle = thread::spawn(move || processor(value));
    handle.join().unwrap()
}
```

## Explanation

This solution demonstrates **generic functions with Send constraints** for safe thread communication:

### Key Concepts Demonstrated:

1. **Send Trait Bounds**:
   - `T: Send + 'static`: The value can be safely moved between threads
   - `F: FnOnce(T) -> T + Send + 'static`: The closure can be moved to another thread
   - These bounds ensure thread safety at compile time

2. **Generic Thread Communication**:
   - Works with any type that implements `Send`
   - Provides a reusable pattern for thread-based processing
   - Type-safe transfer of ownership across thread boundaries

3. **'static Lifetime Requirement**:
   - Spawned threads require `'static` lifetime for their contents
   - Ensures no references to stack-allocated data escape their scope
   - Necessary for thread safety guarantees

4. **Ownership Transfer Pattern**:
   - Value is **moved** into the thread (not borrowed)
   - Processing function **owns** the value during execution
   - Result is **moved** back to the calling thread

### How the Function Works:

```rust
// Step-by-step execution:
let result = send_to_thread(vec![1, 2, 3, 4, 5], |mut v| {
    // 1. vec![1,2,3,4,5] is moved into the spawned thread
    // 2. The closure receives ownership of the vector
    // 3. Processing happens in the spawned thread
    v.iter_mut().for_each(|x| *x *= 2);
    // 4. Modified vector is returned from closure
    v
    // 5. Result is moved back to calling thread via join()
});
```

### Send Trait Requirements:

For a type to be `Send`:
- **Primitive types**: `i32`, `String`, `Vec<T>` (when `T: Send`)
- **Owned data**: `Box<T>`, unique ownership patterns
- **Thread-safe types**: `Arc<Mutex<T>>`, `mpsc::Sender<T>`

Not `Send`:
- **Reference-counted**: `Rc<T>`, `Weak<T>`  
- **Raw pointers**: `*const T`, `*mut T`
- **Thread-local**: Types with thread-local storage

### Function Signature Breakdown:

```rust
pub fn send_to_thread<
    T: Send + 'static,              // Value type constraints
    F: FnOnce(T) -> T + Send + 'static  // Closure type constraints
>(
    value: T,      // Owned value to process
    processor: F,  // Processing function
) -> T {          // Processed result
```

### Why These Constraints Matter:

1. **T: Send**: Value can be moved between threads safely
2. **T: 'static**: No borrowed references that could outlive the thread
3. **F: Send**: Closure can be moved to another thread
4. **F: 'static**: Closure doesn't capture short-lived references
5. **FnOnce**: Closure takes ownership of its parameters

### Usage Examples:

```rust
// Example 1: Processing numbers
let doubled = send_to_thread(vec![1, 2, 3], |v| {
    v.into_iter().map(|x| x * 2).collect()
});

// Example 2: String manipulation  
let processed = send_to_thread("hello".to_string(), |s| {
    s.to_uppercase()
});

// Example 3: Complex data structures
let result = send_to_thread(
    MyStruct { data: vec![1, 2, 3] },
    |mut s| {
        s.data.push(4);
        s
    }
);
```

### Error Cases (Won't Compile):

```rust
// ERROR: Rc is not Send
let rc_data = Rc::new(42);
send_to_thread(rc_data, |x| x); // Compile error

// ERROR: Borrowed reference isn't 'static  
let local_var = 42;
send_to_thread(5, |x| x + local_var); // Compile error

// ERROR: Raw pointer isn't Send
let ptr = &42 as *const i32;
send_to_thread(ptr, |p| p); // Compile error
```

### Performance Considerations:

- **Thread Creation Overhead**: Creating threads has some cost
- **Context Switching**: OS overhead for thread scheduling
- **Data Movement**: Large data structures may be expensive to move
- **Synchronization**: `join()` blocks until thread completes

### Alternative Patterns:

```rust
// For CPU-bound work:
use std::thread;

// For I/O-bound work:
use tokio::task;

// For work-stealing:
use rayon::prelude::*;
```

### Real-world Applications:

1. **Background Processing**: Offloading expensive computations
2. **Parallel Algorithms**: Map-reduce style operations
3. **Isolation**: Processing untrusted data in separate threads
4. **Resource Management**: Thread-specific resource handling

### Thread Safety Guarantees:

The type system ensures:
- **No data races**: Only owned data crosses thread boundaries
- **Memory safety**: No use-after-free or double-free
- **Type safety**: Generic constraints prevent unsafe operations

This utility function demonstrates how Rust's type system enables safe, generic thread communication patterns while preventing common concurrency bugs at compile time.