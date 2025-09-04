# Interior Mutability - Solution

## Solution

```rust
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::collections::HashMap;

pub struct Counter {
    count: RefCell<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count: RefCell::new(0),
        }
    }
    
    pub fn increment(&self) -> usize {
        let mut count = self.count.borrow_mut();
        *count += 1;
        *count
    }
    
    pub fn get(&self) -> usize {
        *self.count.borrow()
    }
    
    pub fn add(&self, amount: usize) -> usize {
        let mut count = self.count.borrow_mut();
        *count += amount;
        *count
    }
    
    pub fn reset(&self) {
        *self.count.borrow_mut() = 0;
    }
}

#[derive(Debug)]
pub struct SharedList<T> {
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> SharedList<T> {
    pub fn new() -> Self {
        SharedList {
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    pub fn from_vec(vec: Vec<T>) -> Self {
        SharedList {
            data: Rc::new(RefCell::new(vec)),
        }
    }
    
    pub fn clone_handle(&self) -> Self {
        SharedList {
            data: Rc::clone(&self.data),
        }
    }
    
    pub fn push(&self, item: T) {
        self.data.borrow_mut().push(item);
    }
    
    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }
    
    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
    
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.data.borrow().get(index).cloned()
    }
    
    pub fn for_each<F>(&self, mut func: F)
    where
        F: FnMut(&T),
    {
        for item in self.data.borrow().iter() {
            func(item);
        }
    }
    
    pub fn map_in_place<F>(&self, func: F)
    where
        F: Fn(&T) -> T,
        T: Clone,
    {
        let mut data = self.data.borrow_mut();
        for item in data.iter_mut() {
            *item = func(item);
        }
    }
}

pub struct MutableCache<K, V> {
    cache: RefCell<HashMap<K, V>>,
}

impl<K, V> MutableCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        MutableCache {
            cache: RefCell::new(HashMap::new()),
        }
    }
    
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        {
            let cache = self.cache.borrow();
            if let Some(value) = cache.get(&key) {
                return value.clone();
            }
        }
        
        let value = compute();
        self.cache.borrow_mut().insert(key, value.clone());
        value
    }
    
    pub fn insert(&self, key: K, value: V) {
        self.cache.borrow_mut().insert(key, value);
    }
    
    pub fn contains_key(&self, key: &K) -> bool {
        self.cache.borrow().contains_key(key)
    }
    
    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }
    
    pub fn len(&self) -> usize {
        self.cache.borrow().len()
    }
}

pub struct CellCounter {
    count: Cell<i32>,
}

impl CellCounter {
    pub fn new(initial: i32) -> Self {
        CellCounter {
            count: Cell::new(initial),
        }
    }
    
    pub fn get(&self) -> i32 {
        self.count.get()
    }
    
    pub fn set(&self, value: i32) {
        self.count.set(value);
    }
    
    pub fn increment(&self) -> i32 {
        let current = self.count.get();
        self.count.set(current + 1);
        current
    }
    
    pub fn add(&self, amount: i32) -> i32 {
        let new_value = self.count.get() + amount;
        self.count.set(new_value);
        new_value
    }
}

pub struct MockService {
    call_log: RefCell<Vec<String>>,
}

impl MockService {
    pub fn new() -> Self {
        MockService {
            call_log: RefCell::new(Vec::new()),
        }
    }
    
    pub fn call_method(&self, method_name: &str, args: &str) -> String {
        let call_entry = format!("{}({})", method_name, args);
        self.call_log.borrow_mut().push(call_entry);
        format!("Mock response for {}", method_name)
    }
    
    pub fn call_count(&self) -> usize {
        self.call_log.borrow().len()
    }
    
    pub fn get_call_log(&self) -> Vec<String> {
        self.call_log.borrow().clone()
    }
    
    pub fn reset(&self) {
        self.call_log.borrow_mut().clear();
    }
    
    pub fn was_called(&self, method_name: &str) -> bool {
        self.call_log
            .borrow()
            .iter()
            .any(|call| call.contains(method_name))
    }
}

pub fn demonstrate_borrow_checking() -> Result<String, &'static str> {
    let cell = RefCell::new(String::from("test"));
    
    // Try to create conflicting borrows
    let _borrow1 = cell.borrow();
    
    // This would panic at runtime if we tried to borrow_mut while borrow1 is active
    match std::panic::catch_unwind(|| {
        let _borrow2 = cell.borrow_mut();
        "Should not reach here".to_string()
    }) {
        Ok(_) => Ok("No conflict detected".to_string()),
        Err(_) => Err("Borrow checking prevented conflict"),
    }
}
```

## Explanation

### Interior Mutability Concepts

**The Problem Interior Mutability Solves:**
```rust
// Without interior mutability, this is impossible:
fn modify_through_immutable_ref(data: &SomeStruct) {
    // data.field = new_value;  // Compile error!
}

// With RefCell, this works:
fn modify_through_refcell(data: &RefCell<SomeStruct>) {
    data.borrow_mut().field = new_value;  // Runtime borrowing
}
```

**Key Insight:** Interior mutability moves borrow checking from compile-time to runtime, enabling mutation through immutable references.

### RefCell<T> - Runtime Borrow Checking

**Core Operations:**
```rust
let cell = RefCell::new(value);
let borrowed = cell.borrow();        // Immutable borrow (can have multiple)
let mut borrowed = cell.borrow_mut(); // Mutable borrow (exclusive)
```

**Borrow Rules (Enforced at Runtime):**
- Multiple immutable borrows allowed simultaneously
- Only one mutable borrow allowed at a time
- No mutable and immutable borrows simultaneously
- Violations cause panic, not compile errors

**Safe Usage Patterns:**
```rust
pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V {
    // Check if exists with immutable borrow
    {
        let cache = self.cache.borrow();
        if let Some(value) = cache.get(&key) {
            return value.clone();
        }
    } // Immutable borrow dropped here
    
    // Insert with mutable borrow
    let value = compute();
    self.cache.borrow_mut().insert(key, value.clone());
    value
}
```

### Cell<T> - Simple Interior Mutability

**For Copy Types Only:**
```rust
pub struct CellCounter {
    count: Cell<i32>,  // i32 implements Copy
}
```

**Key Differences from RefCell<T>:**
- No borrowing - uses `get()` and `set()`
- Only works with Copy types
- Cannot return references to contained data
- No runtime borrow checking overhead
- Cannot panic from borrowing conflicts

**When to Use:**
- Simple primitive values that need interior mutability
- Performance-critical code where RefCell overhead matters
- When you only need get/set operations, not borrowing

### Rc<RefCell<T>> Pattern

**Shared Mutable Ownership:**
```rust
#[derive(Debug)]
pub struct SharedList<T> {
    data: Rc<RefCell<Vec<T>>>,  // Multiple owners, interior mutability
}

pub fn clone_handle(&self) -> Self {
    SharedList {
        data: Rc::clone(&self.data),  // Share the same RefCell
    }
}
```

**Common Pattern Benefits:**
- Multiple owners can all mutate the same data
- Automatic cleanup when all owners are dropped
- Runtime safety through borrow checking

### Practical Applications

**Caching with Interior Mutability:**
```rust
// Cache can be modified even through &self
pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V {
    // Implementation uses RefCell for interior mutability
}
```

**Mock Objects for Testing:**
```rust
pub struct MockService {
    call_log: RefCell<Vec<String>>,  // Track calls through immutable refs
}

impl MockService {
    pub fn call_method(&self, method_name: &str, args: &str) -> String {
        // Can log calls even with &self
        self.call_log.borrow_mut().push(call_entry);
    }
}
```

### Runtime Safety and Error Handling

**Borrow Checking Panics:**
```rust
let cell = RefCell::new(42);
let borrow1 = cell.borrow();
let borrow2 = cell.borrow_mut(); // Panic! Conflicting borrows
```

**Safe Patterns:**
1. **Scope Management** - Drop borrows quickly with explicit scopes
2. **Check Before Borrow** - Use `try_borrow()` and `try_borrow_mut()`
3. **Single Responsibility** - Keep borrow scopes focused and minimal

### Performance Considerations

**RefCell<T> Overhead:**
- Runtime borrow checking adds small overhead
- Borrow counter storage in each RefCell
- Dynamic checks on every borrow operation

**Cell<T> Performance:**
- No borrowing overhead
- Simple get/set operations
- Best for frequently accessed simple values

**Memory Layout:**
```rust
RefCell<T>:  [borrow_flag] + [T]
Cell<T>:     [T]            // No extra overhead
Rc<RefCell<T>>: [ref_count] + [weak_count] + [borrow_flag] + [T]
```

### Best Practices

**Design Guidelines:**
1. **Prefer compile-time borrowing** when possible
2. **Use Cell<T>** for simple Copy types
3. **Use RefCell<T>** when you need to borrow data
4. **Keep borrow scopes minimal** to avoid panics
5. **Consider Arc<Mutex<T>>** for thread-safe interior mutability

**Common Anti-Patterns:**
- Long-lived borrows that increase panic risk
- Using RefCell when regular borrowing would work
- Storing borrows in data structures (causes lifetime issues)
- Using interior mutability as default instead of last resort

**Testing Interior Mutability:**
- Mock objects that track interactions
- Caches that can be inspected and cleared
- Counters and metrics that update through shared references
- Observer patterns where subjects notify through immutable interfaces