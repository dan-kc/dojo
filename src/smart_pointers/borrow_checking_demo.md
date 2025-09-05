# Borrow Checking Demo - Solution

## Solution

```rust
use std::cell::RefCell;

/// Demonstrate potential runtime panics with RefCell borrow checking
pub fn demonstrate_borrow_checking() -> Result<String, &'static str> {
    let cell = RefCell::new(String::from("test_data"));
    
    // Try to create conflicting borrows safely
    let _immutable_borrow = cell.borrow();
    
    // Use panic catching to handle runtime borrow conflicts gracefully
    let result = std::panic::catch_unwind(|| {
        let _mutable_borrow = cell.borrow_mut(); // This will panic!
        String::from("Should never reach here")
    });
    
    match result {
        Ok(_) => Ok("No borrow conflict detected".to_string()),
        Err(_) => Err("Borrow checking prevented conflict - RefCell detected simultaneous immutable and mutable borrows"),
    }
}

/// Safe wrapper that attempts operations and reports success/failure
pub fn safe_refcell_operations() -> Vec<String> {
    let cell = RefCell::new(vec![1, 2, 3, 4, 5]);
    let mut results = Vec::new();
    
    // 1. Successful immutable borrow
    match cell.try_borrow() {
        Ok(borrowed) => {
            results.push(format!("Success: Immutable borrow, length = {}", borrowed.len()));
        }
        Err(_) => {
            results.push("Fail: Could not create immutable borrow".to_string());
        }
    }
    
    // 2. Successful mutable borrow (previous borrow was dropped)
    match cell.try_borrow_mut() {
        Ok(mut borrowed) => {
            borrowed.push(6);
            results.push(format!("Success: Mutable borrow, added element, new length = {}", borrowed.len()));
        }
        Err(_) => {
            results.push("Fail: Could not create mutable borrow".to_string());
        }
    }
    
    // 3. Demonstrate borrow conflict
    let _long_lived_borrow = cell.borrow(); // Hold this borrow
    match cell.try_borrow_mut() {
        Ok(_) => {
            results.push("Unexpected success: Mutable borrow while immutable borrow active".to_string());
        }
        Err(_) => {
            results.push("Success: RefCell correctly prevented conflicting mutable borrow".to_string());
        }
    }
    // _long_lived_borrow is dropped here
    
    // 4. Now mutable borrow should work again
    match cell.try_borrow_mut() {
        Ok(mut borrowed) => {
            borrowed.push(7);
            results.push("Success: Mutable borrow worked after immutable borrow was released".to_string());
        }
        Err(_) => {
            results.push("Fail: Unexpected borrow conflict".to_string());
        }
    }
    
    // 5. Multiple immutable borrows are allowed
    let _borrow1 = cell.borrow();
    let _borrow2 = cell.borrow();
    let _borrow3 = cell.borrow();
    results.push("Success: Multiple simultaneous immutable borrows allowed".to_string());
    
    results
}
```

## Explanation

### RefCell Runtime Borrow Checking

**Compile-Time vs Runtime Borrowing:**
```rust
// Compile-time borrowing (normal Rust)
let mut data = vec![1, 2, 3];
let ref1 = &data;       // Immutable borrow
let ref2 = &mut data;   // COMPILE ERROR: cannot borrow as mutable

// Runtime borrowing (RefCell)
let cell = RefCell::new(vec![1, 2, 3]);
let ref1 = cell.borrow();     // Immutable borrow - OK at runtime
let ref2 = cell.borrow_mut(); // RUNTIME PANIC: borrow conflict!
```

**Key Insight:** RefCell defers borrow checking to runtime, enabling patterns impossible with compile-time borrowing, but at the cost of potential runtime panics.

### Borrow Rules at Runtime

**RefCell maintains the same rules as compile-time borrowing:**
1. **Multiple immutable borrows:** ✅ Allowed simultaneously
2. **Single mutable borrow:** ✅ Exclusive access when no other borrows exist
3. **Mixed borrows:** ❌ Cannot have immutable and mutable borrows simultaneously
4. **Multiple mutable borrows:** ❌ Cannot have more than one mutable borrow

**Runtime Enforcement:**
```rust
// RefCell internally tracks:
// - Number of active immutable borrows (0 to many)
// - Number of active mutable borrows (0 or 1)
// - Panics when rules are violated

impl<T> RefCell<T> {
    fn borrow(&self) -> Ref<T> {
        if self.borrow_flag.get() == WRITING {
            panic!("already mutably borrowed");
        }
        // Increment immutable borrow counter
    }
    
    fn borrow_mut(&self) -> RefMut<T> {
        if self.borrow_flag.get() != UNUSED {
            panic!("already borrowed");
        }
        // Set to exclusive mutable borrow state
    }
}
```

### Safe Borrow Handling

**The Problem with Panics:**
```rust
// This code can panic at runtime
fn dangerous_operation(cell: &RefCell<Vec<i32>>) {
    let _immutable = cell.borrow();
    let _mutable = cell.borrow_mut(); // PANIC!
    // Program crashes here
}
```

**Safe Alternatives:**
```rust
// 1. Use try_borrow() for error handling
fn safe_operation(cell: &RefCell<Vec<i32>>) -> Result<(), &'static str> {
    let _immutable = cell.borrow();
    match cell.try_borrow_mut() {
        Ok(_mutable) => Ok(()),
        Err(_) => Err("Could not acquire mutable borrow"),
    }
}

// 2. Use panic::catch_unwind() for panic recovery
fn recovered_operation(cell: &RefCell<Vec<i32>>) -> Result<String, String> {
    std::panic::catch_unwind(|| {
        let _immutable = cell.borrow();
        let _mutable = cell.borrow_mut();
        "Success".to_string()
    }).map_err(|_| "Borrow conflict occurred".to_string())
}
```

### Borrow Lifecycle Management

**Understanding Borrow Guards:**
```rust
let cell = RefCell::new(vec![1, 2, 3]);

{
    let borrowed = cell.borrow(); // Ref<Vec<i32>> created
    println!("Length: {}", borrowed.len());
    // borrowed implements Deref to access Vec methods
} // Ref<Vec<i32>> dropped here - borrow released

// Now it's safe to borrow mutably
let mut borrowed = cell.borrow_mut(); // RefMut<Vec<i32>>
borrowed.push(4);
// RefMut<Vec<i32>> dropped at end of scope
```

**Common Borrow Lifetime Issues:**
```rust
// BAD: Trying to store borrow guards
struct BadExample<'a> {
    data: Ref<'a, Vec<i32>>, // Lifetime nightmare!
}

// GOOD: Extract what you need from borrows
struct GoodExample {
    length: usize,
    first_element: Option<i32>,
}

impl GoodExample {
    fn from_cell(cell: &RefCell<Vec<i32>>) -> Self {
        let borrowed = cell.borrow();
        GoodExample {
            length: borrowed.len(),
            first_element: borrowed.first().cloned(),
        }
        // borrowed dropped here
    }
}
```

### Advanced Conflict Detection

**Borrow State Introspection:**
```rust
use std::cell::{RefCell, BorrowState};

fn analyze_borrow_state<T>(cell: &RefCell<T>) -> String {
    match cell.try_borrow() {
        Ok(_) => {
            match cell.try_borrow_mut() {
                Ok(_) => "No active borrows".to_string(),
                Err(_) => "Has immutable borrows".to_string(),
            }
        }
        Err(_) => "Has mutable borrow".to_string(),
    }
}

// Usage in diagnostics
fn debug_borrow_conflicts(cell: &RefCell<Vec<i32>>) {
    println!("State before operation: {}", analyze_borrow_state(cell));
    
    let _borrow = cell.borrow();
    println!("State with immutable borrow: {}", analyze_borrow_state(cell));
    
    drop(_borrow);
    println!("State after dropping borrow: {}", analyze_borrow_state(cell));
}
```

### Error Recovery Patterns

**Graceful Degradation:**
```rust
struct ResilientCache {
    data: RefCell<HashMap<String, String>>,
}

impl ResilientCache {
    fn get_or_default(&self, key: &str) -> String {
        // Try normal operation first
        if let Ok(cache) = self.data.try_borrow() {
            if let Some(value) = cache.get(key) {
                return value.clone();
            }
        }
        
        // Fall back to default if borrow fails
        format!("default_value_for_{}", key)
    }
    
    fn try_insert(&self, key: String, value: String) -> Result<(), &'static str> {
        match self.data.try_borrow_mut() {
            Ok(mut cache) => {
                cache.insert(key, value);
                Ok(())
            }
            Err(_) => Err("Cache temporarily unavailable"),
        }
    }
}
```

**Retry Logic:**
```rust
fn retry_borrow_mut<T, F, R>(
    cell: &RefCell<T>,
    mut operation: F,
    max_attempts: usize,
) -> Result<R, &'static str>
where
    F: FnMut(&mut T) -> R,
{
    for attempt in 0..max_attempts {
        match cell.try_borrow_mut() {
            Ok(mut borrowed) => return Ok(operation(&mut *borrowed)),
            Err(_) if attempt + 1 < max_attempts => {
                // Brief pause before retry
                std::thread::sleep(std::time::Duration::from_micros(10));
            }
            Err(_) => return Err("Max retry attempts exceeded"),
        }
    }
    unreachable!()
}
```

### Testing Borrow Behavior

**Positive Testing - Successful Operations:**
```rust
#[test]
fn test_successful_borrows() {
    let cell = RefCell::new(vec![1, 2, 3]);
    
    // Sequential borrows should work
    {
        let immutable = cell.borrow();
        assert_eq!(immutable.len(), 3);
    }
    
    {
        let mut mutable = cell.borrow_mut();
        mutable.push(4);
        assert_eq!(mutable.len(), 4);
    }
    
    // Multiple immutable borrows
    let _borrow1 = cell.borrow();
    let _borrow2 = cell.borrow();
    let _borrow3 = cell.borrow();
    // All should coexist peacefully
}
```

**Negative Testing - Borrow Conflicts:**
```rust
#[test]
#[should_panic(expected = "already borrowed")]
fn test_immutable_then_mutable_panic() {
    let cell = RefCell::new(vec![1, 2, 3]);
    let _immutable = cell.borrow();
    let _mutable = cell.borrow_mut(); // Should panic
}

#[test]
#[should_panic(expected = "already mutably borrowed")]
fn test_mutable_then_immutable_panic() {
    let cell = RefCell::new(vec![1, 2, 3]);
    let _mutable = cell.borrow_mut();
    let _immutable = cell.borrow(); // Should panic
}

#[test]
fn test_try_borrow_handles_conflicts_gracefully() {
    let cell = RefCell::new(vec![1, 2, 3]);
    let _immutable = cell.borrow();
    
    // This should not panic
    assert!(cell.try_borrow_mut().is_err());
}
```

### Performance Impact of Borrow Checking

**Runtime Overhead:**
```rust
// Each borrow operation has overhead:
// 1. Check current borrow state
// 2. Update borrow counters
// 3. Create borrow guard
// 4. Handle drop cleanup

// Microbenchmark comparison:
fn direct_access(data: &mut Vec<i32>) {
    data.push(42); // ~1-2 CPU cycles
}

fn refcell_access(cell: &RefCell<Vec<i32>>) {
    cell.borrow_mut().push(42); // ~10-20 CPU cycles
}
```

**Optimization Strategies:**
```rust
// BAD: Multiple short borrows
for i in 0..1000 {
    cell.borrow_mut().push(i); // Overhead on each iteration
}

// GOOD: Single long-lived borrow
{
    let mut borrowed = cell.borrow_mut();
    for i in 0..1000 {
        borrowed.push(i); // Direct access after initial borrow
    }
} // Borrow released once
```

### Real-World Debugging

**Common Panic Messages and Solutions:**
```rust
// "already borrowed: BorrowMutError"
// Solution: Check for overlapping immutable and mutable borrows
let _immutable = cell.borrow();
let _mutable = cell.borrow_mut(); // ← Problem here

// "already mutably borrowed: BorrowError"  
// Solution: Check for multiple mutable borrows or mutable+immutable
let _mutable1 = cell.borrow_mut();
let _mutable2 = cell.borrow_mut(); // ← Problem here

// "thread 'main' panicked at 'already borrowed'"
// Solution: Use try_borrow() instead of borrow()
match cell.try_borrow() {
    Ok(borrowed) => { /* use borrowed */ }
    Err(_) => { /* handle gracefully */ }
}
```

**Debugging Tools:**
```rust
// Custom wrapper for debugging borrow issues
struct DebugRefCell<T> {
    cell: RefCell<T>,
    label: String,
}

impl<T> DebugRefCell<T> {
    fn borrow(&self) -> std::cell::Ref<T> {
        println!("Attempting immutable borrow on {}", self.label);
        match self.cell.try_borrow() {
            Ok(borrowed) => {
                println!("✓ Successful immutable borrow on {}", self.label);
                borrowed
            }
            Err(_) => {
                println!("✗ Failed immutable borrow on {} - conflicts exist", self.label);
                panic!("Borrow conflict in {}", self.label);
            }
        }
    }
}
```

### Best Practices for Borrow Safety

**Design Guidelines:**
1. **Keep borrows short-lived:** Release as quickly as possible
2. **Use try_borrow() in production:** Handle conflicts gracefully
3. **Avoid storing borrow guards:** Extract data instead
4. **Use explicit scopes:** Control borrow lifetimes precisely
5. **Consider alternatives:** Regular borrowing, Arc<Mutex<T>>, or redesign

**Code Review Checklist:**
- [ ] Are all RefCell borrows released promptly?
- [ ] Are there any potential overlapping borrow conflicts?
- [ ] Is error handling in place for borrow failures?
- [ ] Could the design use regular borrowing instead?
- [ ] Are borrow scopes minimized and explicit?

RefCell's runtime borrow checking demonstrates how Rust can maintain memory safety even when compile-time analysis isn't sufficient, but requires careful programming to handle the runtime failure cases gracefully.