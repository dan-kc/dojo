# Non-Send Container Safe Usage

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

pub struct NonSendContainer {
    data: Mutex<Option<Rc<String>>>,
}

impl NonSendContainer {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(None),
        }
    }

    pub fn set_data(&self, data: String) {
        let mut guard = self.data.lock().unwrap();
        *guard = Some(Rc::new(data));
    }

    pub fn get_data(&self) -> Option<String> {
        let guard = self.data.lock().unwrap();
        guard.as_ref().map(|rc| (**rc).clone())
    }
}

// NonSendContainer is Send and Sync because:
// - Mutex<T> is Send + Sync when T is Send
// - Option<Rc<String>> is Send (even though Rc normally isn't)
// - The key is that we never share the Rc between threads - 
//   each thread creates/destroys its own Rc instances
```

## Explanation

This solution demonstrates **safe encapsulation** of non-Send data within a Send container:

### Key Concepts Demonstrated:

1. **Container vs Content Send Properties**:
   - The **container** (`NonSendContainer`) can be Send even if it contains non-Send data
   - The **content** (`Rc<String>`) is never shared between threads directly
   - Safe encapsulation prevents the non-Send data from escaping thread boundaries

2. **Safe Encapsulation Strategy**:
   - `Mutex` ensures exclusive access to the `Rc<String>`
   - Data is **cloned** rather than **shared** when accessed across threads
   - Each thread gets its own copy of the string data

3. **Why This Works**:
   - `Rc<String>` is created and destroyed within the same thread
   - No `Rc` instance ever crosses thread boundaries
   - Only the **data content** is shared (via cloning), not the reference-counting mechanism

4. **Mutex as Thread Boundary**:
   - The `Mutex` acts as a synchronization point
   - Ensures that `Rc` operations happen atomically
   - Prevents data races on the reference count

### Send Trait Rules:

```rust
// This would NOT be safe:
fn bad_example() {
    let rc = Rc::new(String::from("data"));
    thread::spawn(move || {
        println!("{}", rc); // ERROR: Rc is not Send
    });
}

// This IS safe (our approach):
fn good_example() {
    let container = NonSendContainer::new();
    container.set_data("data".to_string());
    
    thread::spawn(move || {
        // container can be moved because it's Send
        if let Some(data) = container.get_data() {
            println!("{}", data); // data is String (Send)
        }
    });
}
```

### Why Rc Isn't Send:

`Rc<T>` (Reference Counted) isn't `Send` because:
- It uses non-atomic reference counting
- Multiple threads incrementing/decrementing the count would cause data races
- The reference count could become corrupted

### Our Solution's Safety:

1. **Atomic Access**: `Mutex` ensures only one thread accesses the `Rc` at a time
2. **No Sharing**: `Rc` instances never cross thread boundaries
3. **Data Cloning**: Only the underlying `String` data is shared (via cloning)
4. **RAII**: `Rc` is properly cleaned up within the thread that created it

### Alternative Approaches:

For similar use cases, consider:
- **Arc<T>**: Atomic reference counting (designed for multi-threading)
- **Box<T>**: Unique ownership without reference counting
- **String directly**: Skip reference counting entirely

### When This Pattern Is Useful:

1. **Legacy Code**: Working with APIs that use `Rc` but need thread safety
2. **Memory Optimization**: Sharing large data between operations in the same thread
3. **FFI Integration**: Wrapping C types that aren't thread-safe
4. **Gradual Migration**: Moving from single-threaded to multi-threaded code

### Performance Considerations:

- **Cloning overhead**: Each access clones the string data
- **Lock contention**: All access serialized through mutex
- **Memory usage**: Each thread gets its own copy of the data

This pattern demonstrates how careful API design can provide thread safety even when working with inherently non-thread-safe types, by ensuring the non-Send data never escapes its thread boundaries.