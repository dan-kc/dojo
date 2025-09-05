# Shared List - Solution

## Solution

```rust
use std::cell::RefCell;
use std::rc::Rc;

/// Create a shared mutable list using Rc<RefCell<Vec<T>>>.
/// Multiple owners can modify the same list.
#[derive(Debug)]
pub struct SharedList<T> {
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> SharedList<T> {
    /// Create a new empty shared list
    pub fn new() -> Self {
        SharedList {
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }
    
    /// Create a new shared list from existing data
    pub fn from_vec(vec: Vec<T>) -> Self {
        SharedList {
            data: Rc::new(RefCell::new(vec)),
        }
    }
    
    /// Clone the shared list (creates new handle to same data)
    pub fn clone_handle(&self) -> Self {
        SharedList {
            data: Rc::clone(&self.data),
        }
    }
    
    /// Push an item to the list
    pub fn push(&self, item: T) {
        self.data.borrow_mut().push(item);
    }
    
    /// Pop an item from the list
    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }
    
    /// Get the length of the list
    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }
    
    /// Get a copy of the item at index (if T: Clone)
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.data.borrow().get(index).cloned()
    }
    
    /// Apply a function to each element in the list
    pub fn for_each<F>(&self, mut func: F)
    where
        F: FnMut(&T),
    {
        for item in self.data.borrow().iter() {
            func(item);
        }
    }
    
    /// Transform each element in the list using a function
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
```

## Explanation

### Rc<RefCell<T>> Pattern - Shared Mutable Ownership

**The Problem This Pattern Solves:**
```rust
// This doesn't work - can't have multiple owners of mutable data
let vec = Vec::new();
let handle1 = &vec;  // These are just borrowing
let handle2 = &vec;  // Can't give away ownership to both

// This works - multiple owners of the same data
let list = SharedList::new();
let handle1 = list.clone_handle();  // Each gets their own Rc
let handle2 = list.clone_handle();  // All point to same data
```

**Key Insight:** `Rc<RefCell<T>>` combines shared ownership (`Rc<T>`) with interior mutability (`RefCell<T>`), enabling multiple owners to mutate the same data safely.

### Understanding the Components

**Rc<T> - Reference Counting:**
```rust
Rc::new(data)           // Creates new Rc with reference count = 1
Rc::clone(&rc)          // Increments reference count, shares data
// When last Rc is dropped, data is automatically freed
```

**RefCell<T> - Interior Mutability:**
```rust
RefCell::new(data)      // Wraps data with runtime borrow checking
cell.borrow()           // Immutable borrow (multiple allowed)
cell.borrow_mut()       // Mutable borrow (exclusive)
```

**Combined Power:**
```rust
Rc<RefCell<Vec<T>>>  // Multiple owners + safe mutation
//    │       └─ The actual data (Vec<T>)
//    └─ Runtime borrowing rules
└─ Reference counting for shared ownership
```

### Implementation Deep Dive

**Memory Layout Visualization:**
```rust
SharedList<i32> list1 = SharedList::new();
SharedList<i32> list2 = list1.clone_handle();

// Memory layout:
// list1.data ──┐
//              ├─► [Rc: count=2] ──► [RefCell: borrow_flag] ──► [Vec: [1,2,3]]
// list2.data ──┘
```

**Clone Handle vs Clone Data:**
```rust
// clone_handle() - shares the same data
pub fn clone_handle(&self) -> Self {
    SharedList {
        data: Rc::clone(&self.data),  // Increments Rc count
    }
}

// This would clone the actual data (expensive!)
pub fn clone_data(&self) -> Self {
    SharedList {
        data: Rc::new(RefCell::new(self.data.borrow().clone())),
    }
}
```

**Safe Borrowing Patterns:**

1. **Short-lived Borrows:**
```rust
pub fn push(&self, item: T) {
    self.data.borrow_mut().push(item);  // Borrow, modify, immediately drop
}
```

2. **Read Operations:**
```rust
pub fn len(&self) -> usize {
    self.data.borrow().len()  // Immutable borrow for read-only access
}
```

3. **Complex Operations:**
```rust
pub fn map_in_place<F>(&self, func: F) {
    let mut data = self.data.borrow_mut();  // Single borrow for entire operation
    for item in data.iter_mut() {
        *item = func(item);
    }
    // Borrow released when data goes out of scope
}
```

### Advanced Usage Patterns

**Functional Operations:**
```rust
pub fn for_each<F>(&self, mut func: F)
where
    F: FnMut(&T),  // Note: FnMut allows the closure to mutate its environment
{
    for item in self.data.borrow().iter() {
        func(item);  // Each item passed to closure
    }
}

// Usage example:
let mut sum = 0;
list.for_each(|x| sum += x);  // Closure captures and mutates sum
```

**In-Place Transformations:**
```rust
pub fn map_in_place<F>(&self, func: F)
where
    F: Fn(&T) -> T,  // Pure function transformation
    T: Clone,
{
    let mut data = self.data.borrow_mut();
    for item in data.iter_mut() {
        *item = func(item);  // Transform each element
    }
}

// Usage:
list.map_in_place(|x| x * 2);  // Double all values
```

### Memory Management and Cleanup

**Automatic Cleanup:**
```rust
{
    let list1 = SharedList::from_vec(vec![1, 2, 3]);
    let list2 = list1.clone_handle();  // Rc count = 2
    
    // list1 goes out of scope, Rc count = 1
}
// list2 goes out of scope, Rc count = 0, Vec is dropped
```

**Reference Cycle Prevention:**
SharedList doesn't create cycles naturally, but be careful with complex structures:
```rust
// This could create a cycle:
// list1 ──► vec1 containing list2
// list2 ──► vec2 containing list1
// Solution: Use Weak<RefCell<Vec<T>>> for one direction
```

### Performance Considerations

**Memory Overhead:**
```rust
Vec<i32>:                    [ptr, len, capacity] + heap data
RefCell<Vec<i32>>:          [borrow_flag] + [ptr, len, capacity] + heap data  
Rc<RefCell<Vec<i32>>>:      [strong_count, weak_count] + [borrow_flag] + [ptr, len, capacity] + heap data
SharedList<i32>:            [Rc pointer] → above structure

Total overhead: ~24-32 bytes per SharedList instance
```

**Runtime Performance:**
- **Rc clone:** O(1) - just increment counter
- **RefCell borrow:** O(1) - check and update borrow flag
- **Vector operations:** Same as regular Vec<T>

**When Overhead Matters:**
```rust
// High-frequency operations
for i in 0..1_000_000 {
    list.push(i);  // Each push has borrow overhead
}

// Better: batch operations
{
    let mut data = list.data.borrow_mut();  // Single borrow
    for i in 0..1_000_000 {
        data.push(i);  // Direct vector operations
    }
}  // Borrow released
```

### Error Handling and Safety

**Borrow Conflicts:**
```rust
let list = SharedList::new();
let _immutable_ref = list.data.borrow();
// let _mutable_ref = list.data.borrow_mut();  // Panic!
```

**Safe Patterns:**
```rust
// Pattern 1: Scope management
{
    let data = list.data.borrow();
    println!("Length: {}", data.len());
}  // Borrow released
list.push(42);  // Now safe

// Pattern 2: try_borrow for error handling
match list.data.try_borrow_mut() {
    Ok(mut data) => data.push(42),
    Err(_) => println!("Could not borrow"),
}
```

### Testing Shared Behavior

**Demonstrating Shared Mutation:**
```rust
#[test]
fn test_shared_mutation() {
    let list1 = SharedList::from_vec(vec![1, 2, 3]);
    let list2 = list1.clone_handle();
    
    // Modify through first handle
    list1.push(4);
    assert_eq!(list2.len(), 4);  // Visible through second handle
    
    // Modify through second handle
    list2.push(5);
    assert_eq!(list1.get(4), Some(5));  // Visible through first handle
}
```

### Real-World Applications

**GUI Applications:**
```rust
// Widget tree where multiple components need to modify shared data
struct Widget {
    shared_data: SharedList<Event>,
}

impl Widget {
    fn on_click(&self) {
        self.shared_data.push(Event::Click);
    }
}
```

**Game Development:**
```rust
// Multiple game systems operating on shared entity lists
struct GameWorld {
    entities: SharedList<Entity>,
}

struct PhysicsSystem {
    entities: SharedList<Entity>,  // Same list as GameWorld
}
```

**Data Processing Pipelines:**
```rust
// Multiple processors working on shared data queue
struct DataProcessor {
    work_queue: SharedList<Task>,
}

impl DataProcessor {
    fn process_next(&self) -> Option<Task> {
        self.work_queue.pop()  // All processors share the same queue
    }
}
```

### Best Practices

**Design Guidelines:**
1. **Use sparingly:** Prefer regular ownership when possible
2. **Keep borrows short:** Minimize the scope of borrow_mut()
3. **Batch operations:** Group multiple mutations in single borrow
4. **Consider alternatives:** Arc<Mutex<T>> for thread-safe scenarios

**Common Pitfalls:**
- Storing RefCell borrows in data structures (lifetime issues)
- Long-lived borrows that increase panic risk
- Overusing shared ownership when unique ownership would suffice
- Forgetting that clone_handle() doesn't clone the data itself