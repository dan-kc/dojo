# Send but Not Sync Type Example

## Solution

```rust
use std::cell::RefCell;
use std::thread;

pub struct SendNotSync {
    data: RefCell<i32>,
}

impl SendNotSync {
    pub fn new(value: i32) -> Self {
        Self {
            data: RefCell::new(value),
        }
    }

    pub fn get(&self) -> i32 {
        *self.data.borrow()
    }

    pub fn set(&self, value: i32) {
        *self.data.borrow_mut() = value;
    }
}

// SendNotSync is automatically Send but NOT Sync because:
// - RefCell<T> is Send when T: Send (i32 is Send)
// - RefCell<T> is NOT Sync (even when T: Sync)
// This is because RefCell uses runtime borrow checking which is not thread-safe
```

## Explanation

This solution demonstrates a type that implements **Send but not Sync**, highlighting the distinction between these traits:

### Key Concepts Demonstrated:

1. **Send vs Sync Distinction**:
   - **Send**: Type can be **moved** between threads (ownership transfer)
   - **Sync**: Type can be **shared** between threads (concurrent access to references)
   - A type can be Send without being Sync

2. **RefCell Interior Mutability**:
   - `RefCell<T>` provides interior mutability with runtime borrow checking
   - Safe for single-threaded use but not thread-safe
   - Uses non-atomic reference counting for borrows

3. **Why RefCell is Send but not Sync**:
   - **Send**: The entire `RefCell` can be moved to another thread safely
   - **Not Sync**: Multiple threads cannot safely share references to the same `RefCell`

4. **Runtime Borrow Checking**:
   - `borrow()` and `borrow_mut()` are checked at runtime, not compile time
   - Panics if borrowing rules are violated (e.g., mutable + immutable borrows)
   - This checking mechanism is not atomic/thread-safe

### How Send Without Sync Works:

```rust
// This is ALLOWED (Send):
let send_not_sync = SendNotSync::new(100);
thread::spawn(move || {
    // Entire value is moved to this thread
    send_not_sync.set(200);
    println!("Value: {}", send_not_sync.get());
}); // send_not_sync lives entirely in this thread

// This would NOT COMPILE (not Sync):
let send_not_sync = SendNotSync::new(100);
let shared_ref = &send_not_sync;  
thread::spawn(move || {
    shared_ref.set(200); // ERROR: SendNotSync is not Sync
});
```

### RefCell Borrow Checking:

```rust
let cell = RefCell::new(42);

// These are safe:
let read1 = cell.borrow();     // Immutable borrow
let read2 = cell.borrow();     // Multiple immutable borrows OK
drop(read1);
drop(read2);

let write = cell.borrow_mut(); // Mutable borrow
drop(write);

// This would panic at runtime:
let read = cell.borrow();
let write = cell.borrow_mut(); // PANIC: already borrowed immutably
```

### Thread Safety Analysis:

| Operation | Thread Safe? | Reason |
|-----------|-------------|---------|
| Moving `RefCell` | ✅ Yes | Entire ownership transferred |
| Sharing `&RefCell` | ❌ No | Borrow checking not atomic |
| Multiple `borrow()` | ❌ No | Race conditions possible |
| `borrow_mut()` | ❌ No | Not atomic operation |

### Why This Design Exists:

1. **Single-threaded Performance**: `RefCell` has zero overhead for the common case
2. **Flexibility**: Allows interior mutability without the cost of `Mutex`
3. **API Compatibility**: Enables patterns that require mutable access through `&self`

### Comparison with Alternatives:

| Type | Send | Sync | Use Case |
|------|------|------|----------|
| `RefCell<T>` | ✅ | ❌ | Single-threaded interior mutability |
| `Mutex<T>` | ✅ | ✅ | Multi-threaded interior mutability |  
| `Cell<T>` | ✅ | ❌ | Single-threaded, Copy types only |
| `AtomicI32` | ✅ | ✅ | Lock-free atomic operations |

### Common Patterns with Send-not-Sync Types:

```rust
// Pattern 1: Thread-local processing
fn process_in_thread(data: SendNotSync) {
    thread::spawn(move || {
        // data is owned by this thread
        data.set(data.get() * 2);
        println!("Processed: {}", data.get());
    });
}

// Pattern 2: Builder pattern with thread transfer
struct Builder {
    config: RefCell<Config>,
}

impl Builder {
    fn build_in_thread(self) -> JoinHandle<Config> {
        thread::spawn(move || {
            // self is moved to thread, safe to use RefCell
            self.config.into_inner()
        })
    }
}
```

### Safety Guarantees:

1. **Compile-time Prevention**: Rust prevents sharing `&RefCell` between threads
2. **Runtime Checking**: RefCell panics on borrow rule violations
3. **Memory Safety**: No data races or memory corruption possible

### When to Use Send-not-Sync Types:

- **Worker Threads**: Processing data that needs interior mutability
- **State machines**: Thread-local state that needs flexible mutation
- **Builders**: Constructing objects before sharing them
- **Migration**: Moving single-threaded code to multi-threaded gradually

This pattern showcases Rust's nuanced approach to thread safety, where types can be safely moved between threads while preventing unsafe concurrent access.