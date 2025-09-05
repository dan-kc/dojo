# Cell Counter - Solution

## Solution

```rust
use std::cell::Cell;

/// Demonstrate Cell<T> for Copy types - a simpler alternative to RefCell<T>
pub struct CellCounter {
    count: Cell<i32>,
}

impl CellCounter {
    /// Create a new cell counter
    pub fn new(initial: i32) -> Self {
        CellCounter {
            count: Cell::new(initial),
        }
    }
    
    /// Get the current value
    pub fn get(&self) -> i32 {
        self.count.get()
    }
    
    /// Set a new value
    pub fn set(&self, value: i32) {
        self.count.set(value);
    }
    
    /// Increment the counter and return old value
    pub fn increment(&self) -> i32 {
        let current = self.count.get();
        self.count.set(current + 1);
        current
    }
    
    /// Add to the counter and return new value
    pub fn add(&self, amount: i32) -> i32 {
        let new_value = self.count.get() + amount;
        self.count.set(new_value);
        new_value
    }
}
```

## Explanation

### Cell<T> vs RefCell<T> - When to Use Each

**Key Difference:**
```rust
// RefCell<T> - runtime borrowing for any type
let ref_cell = RefCell::new(vec![1, 2, 3]);
let borrowed = ref_cell.borrow();     // Returns Ref<Vec<i32>>
let value = &borrowed[0];             // Can work with references

// Cell<T> - get/set operations for Copy types only
let cell = Cell::new(42i32);
let value = cell.get();               // Returns i32 (copied)
cell.set(100);                        // Sets new value directly
// No borrowing, no references to interior!
```

**Core Constraint:** Cell<T> only works with types that implement `Copy`. This includes:
- All primitive types: `i32`, `f64`, `bool`, `char`, etc.
- Tuples of Copy types: `(i32, bool)`, `(f32, f32, f32)`, etc.
- Arrays of Copy types: `[i32; 4]`, `[bool; 8]`, etc.

### Why Cell<T> Exists

**Performance Advantage:**
```rust
// RefCell<i32> - has borrowing overhead
RefCell<i32>: [borrow_flag: isize] + [value: i32]
//            ^~8 bytes overhead

// Cell<i32> - no borrowing machinery needed  
Cell<i32>:    [value: i32]
//            ^no overhead!
```

**Safety Advantage:**
```rust
// RefCell can panic at runtime
let ref_cell = RefCell::new(42);
let _borrow1 = ref_cell.borrow();
let _borrow2 = ref_cell.borrow_mut(); // PANIC!

// Cell never panics - no borrowing conflicts possible
let cell = Cell::new(42);
let val1 = cell.get();    // Always safe
cell.set(100);            // Always safe
let val2 = cell.get();    // Always safe
```

### Cell<T> API Deep Dive

**Core Operations:**
```rust
// Creation and basic access
let cell = Cell::new(42);
let value = cell.get();          // Copies the value out
cell.set(100);                   // Replaces the entire value

// Advanced operations
let old_value = cell.replace(200); // Sets new value, returns old
cell.swap(&other_cell);           // Swaps values between two cells

// Taking the value (for non-Copy types in other contexts)
// let value = cell.take(); // Only available for T: Default
```

**Implementation Details:**
```rust
impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        // Safe because T: Copy - we're making a copy, not borrowing
        unsafe { *self.value.get() }
    }
    
    pub fn set(&self, value: T) {
        // Safe because no one can have references to the interior
        unsafe { *self.value.get() = value; }
    }
}
```

### Counter Implementation Patterns

**Post-increment (Return Old Value):**
```rust
pub fn increment(&self) -> i32 {
    let current = self.count.get();    // Read current value
    self.count.set(current + 1);       // Update with new value
    current                            // Return the old value
}

// Usage:
let counter = CellCounter::new(5);
assert_eq!(counter.increment(), 5);   // Returns old value (5)
assert_eq!(counter.get(), 6);         // Now contains 6
```

**Pre-increment (Return New Value):**
```rust
pub fn add(&self, amount: i32) -> i32 {
    let new_value = self.count.get() + amount;  // Calculate new value
    self.count.set(new_value);                  // Update
    new_value                                   // Return new value
}

// Usage:
let counter = CellCounter::new(10);
assert_eq!(counter.add(5), 15);       // Returns new value (15)
assert_eq!(counter.get(), 15);        // Contains 15
```

### Advanced Cell<T> Patterns

**Atomic-like Operations (Without Atomic Overhead):**
```rust
impl CellCounter {
    /// Compare and swap - only update if current value matches expected
    pub fn compare_and_swap(&self, expected: i32, new: i32) -> i32 {
        let current = self.count.get();
        if current == expected {
            self.count.set(new);
        }
        current  // Return actual value (whether swapped or not)
    }
    
    /// Conditional update with closure
    pub fn update_if<F>(&self, condition: F) -> Option<i32>
    where
        F: Fn(i32) -> Option<i32>,
    {
        let current = self.count.get();
        if let Some(new_value) = condition(current) {
            self.count.set(new_value);
            Some(new_value)
        } else {
            None
        }
    }
}

// Usage:
let counter = CellCounter::new(10);

// Only increment if less than 20
counter.update_if(|val| {
    if val < 20 { Some(val + 1) } else { None }
});
```

**Bit Manipulation with Cell:**
```rust
use std::cell::Cell;

struct Flags {
    bits: Cell<u32>,
}

impl Flags {
    fn new() -> Self {
        Flags { bits: Cell::new(0) }
    }
    
    fn set_flag(&self, flag: u32) {
        let current = self.bits.get();
        self.bits.set(current | flag);
    }
    
    fn clear_flag(&self, flag: u32) {
        let current = self.bits.get();
        self.bits.set(current & !flag);
    }
    
    fn has_flag(&self, flag: u32) -> bool {
        (self.bits.get() & flag) != 0
    }
}
```

### When NOT to Use Cell<T>

**Wrong: Non-Copy Types**
```rust
// This won't compile - Vec<T> doesn't implement Copy
// let cell = Cell::new(vec![1, 2, 3]); // ERROR!

// Use RefCell instead:
let ref_cell = RefCell::new(vec![1, 2, 3]);
```

**Wrong: Need References to Interior**
```rust
// Can't do this with Cell - no borrowing
// let cell = Cell::new([1, 2, 3, 4]);
// let slice = &cell.get()[1..3]; // ERROR: temporary value

// Use RefCell if you need references:
let ref_cell = RefCell::new([1, 2, 3, 4]);
let borrowed = ref_cell.borrow();
let slice = &borrowed[1..3];  // Works
```

### Performance Benchmarking

**Cell<T> vs RefCell<T> Performance:**
```rust
// Microbenchmark results (approximate):
fn bench_cell_operations() {
    let cell = Cell::new(0i32);
    
    // ~1-2 CPU cycles per operation
    for i in 0..1_000_000 {
        cell.set(cell.get() + 1);
    }
}

fn bench_refcell_operations() {
    let ref_cell = RefCell::new(0i32);
    
    // ~10-20 CPU cycles per operation (borrow checking overhead)
    for i in 0..1_000_000 {
        *ref_cell.borrow_mut() += 1;
    }
}
```

**Memory Access Patterns:**
```rust
Cell<i32>:     Direct memory access
RefCell<i32>:  Memory access + borrow flag check + atomic operations
Mutex<i32>:    Memory access + OS lock + atomic operations + context switch risk
```

### Thread Safety Considerations

**Cell<T> is NOT Thread-Safe:**
```rust
// This doesn't work across threads:
let counter = Cell::new(0);
std::thread::spawn(|| {
    counter.set(42); // ERROR: Cell doesn't implement Send + Sync
});

// Use AtomicI32 for thread-safe operations:
use std::sync::atomic::{AtomicI32, Ordering};
let atomic = AtomicI32::new(0);
std::thread::spawn(move || {
    atomic.store(42, Ordering::Relaxed); // Works!
});
```

### Real-World Applications

**Configuration Flags:**
```rust
struct Config {
    debug_mode: Cell<bool>,
    max_retries: Cell<u32>,
    timeout_ms: Cell<u64>,
}

impl Config {
    fn toggle_debug(&self) {
        self.debug_mode.set(!self.debug_mode.get());
    }
    
    fn increase_timeout(&self) {
        let current = self.timeout_ms.get();
        self.timeout_ms.set(current * 2);
    }
}
```

**Statistics Tracking:**
```rust
struct Stats {
    requests: Cell<u64>,
    errors: Cell<u64>,
    total_response_time: Cell<u64>,
}

impl Stats {
    fn record_request(&self, response_time: u64, is_error: bool) {
        self.requests.set(self.requests.get() + 1);
        self.total_response_time.set(
            self.total_response_time.get() + response_time
        );
        
        if is_error {
            self.errors.set(self.errors.get() + 1);
        }
    }
    
    fn error_rate(&self) -> f64 {
        let errors = self.errors.get() as f64;
        let total = self.requests.get() as f64;
        if total > 0.0 { errors / total } else { 0.0 }
    }
}
```

**Game State Counters:**
```rust
struct GameState {
    score: Cell<i32>,
    lives: Cell<u8>,
    level: Cell<u8>,
}

impl GameState {
    fn add_points(&self, points: i32) {
        self.score.set(self.score.get() + points);
    }
    
    fn lose_life(&self) -> bool {
        let lives = self.lives.get();
        if lives > 0 {
            self.lives.set(lives - 1);
            true
        } else {
            false  // Game over
        }
    }
}
```

### Best Practices

**Design Guidelines:**
1. **Prefer Cell<T> over RefCell<T>** for Copy types - it's simpler and faster
2. **Use for simple values** - integers, booleans, small tuples, fixed arrays
3. **Avoid for complex logic** - if you need conditional updates, consider RefCell<T>
4. **Consider AtomicT** for thread-safe scenarios instead of Cell<T>

**Performance Tips:**
```rust
// Good: Batch operations
let old_score = score_cell.get();
let new_score = calculate_new_score(old_score);
score_cell.set(new_score);

// Less efficient: Multiple get/set calls
score_cell.set(score_cell.get() + 10);
score_cell.set(score_cell.get() * 2);
score_cell.set(score_cell.get() - 5);
```

**Testing Cell<T>:**
```rust
#[test]
fn test_cell_counter_behavior() {
    let counter = CellCounter::new(100);
    
    // Test post-increment behavior
    assert_eq!(counter.increment(), 100);  // Returns old value
    assert_eq!(counter.get(), 101);        // New value stored
    
    // Test pre-increment behavior  
    assert_eq!(counter.add(10), 111);      // Returns new value
    assert_eq!(counter.get(), 111);        // Same value stored
}
```

Cell<T> provides the simplest form of interior mutability in Rust, trading some flexibility for maximum performance and simplicity. It's the ideal choice for frequently-updated simple values that don't require borrowing or complex mutation patterns.