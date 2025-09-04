# Reference Counting - Solution

## Solution

```rust
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GraphNode<T> {
    value: T,
    children: std::cell::RefCell<Vec<Rc<GraphNode<T>>>>,
    parent: std::cell::RefCell<Option<Weak<GraphNode<T>>>>,
}

impl<T> GraphNode<T> {
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(GraphNode {
            value,
            children: std::cell::RefCell::new(Vec::new()),
            parent: std::cell::RefCell::new(None),
        })
    }
    
    pub fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
        parent.children.borrow_mut().push(child.clone());
        *child.parent.borrow_mut() = Some(Rc::downgrade(parent));
    }
    
    pub fn value(&self) -> &T {
        &self.value
    }
    
    pub fn child_count(&self) -> usize {
        self.children.borrow().len()
    }
    
    pub fn get_child(&self, index: usize) -> Option<Rc<Self>> {
        self.children.borrow().get(index).cloned()
    }
    
    pub fn has_parent(&self) -> bool {
        self.parent.borrow().as_ref()
            .map(|weak| weak.upgrade().is_some())
            .unwrap_or(false)
    }
    
    pub fn get_parent(&self) -> Option<Rc<Self>> {
        self.parent.borrow().as_ref()?.upgrade()
    }
}

pub fn demonstrate_rc_cloning<T>(rc_value: Rc<T>) -> (Rc<T>, usize)
where
    T: Clone,
{
    let cloned_rc = Rc::clone(&rc_value);
    let count = Rc::strong_count(&cloned_rc);
    (cloned_rc, count)
}

#[derive(Debug)]
pub struct SharedCache<K, V> {
    data: HashMap<K, Rc<V>>,
}

impl<K, V> SharedCache<K, V>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        SharedCache {
            data: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Rc<V> {
        let rc_value = Rc::new(value);
        self.data.insert(key, rc_value.clone());
        rc_value
    }
    
    pub fn get(&self, key: &K) -> Option<Rc<V>> {
        self.data.get(key).cloned()
    }
    
    pub fn remove(&mut self, key: &K) -> Option<Rc<V>> {
        self.data.remove(key)
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub fn shared_data_across_threads(data: Vec<String>) -> Vec<usize> {
    let shared_data = Arc::new(data);
    let mut handles = vec![];
    
    for _ in 0..3 {
        let data_clone = Arc::clone(&shared_data);
        let handle = std::thread::spawn(move || {
            data_clone
                .iter()
                .filter(|s| s.contains("rust"))
                .count()
        });
        handles.push(handle);
    }
    
    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect()
}

pub struct CyclicNode<T> {
    value: T,
    next: std::cell::RefCell<Option<Rc<CyclicNode<T>>>>,
    prev: std::cell::RefCell<Option<Weak<CyclicNode<T>>>>,
}

impl<T> CyclicNode<T> {
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(CyclicNode {
            value,
            next: std::cell::RefCell::new(None),
            prev: std::cell::RefCell::new(None),
        })
    }
    
    pub fn connect(first: &Rc<Self>, second: &Rc<Self>) {
        *first.next.borrow_mut() = Some(second.clone());
        *second.prev.borrow_mut() = Some(Rc::downgrade(first));
    }
    
    pub fn get_next(&self) -> Option<Rc<Self>> {
        self.next.borrow().clone()
    }
    
    pub fn get_prev(&self) -> Option<Rc<Self>> {
        self.prev.borrow().as_ref()?.upgrade()
    }
    
    pub fn value(&self) -> &T {
        &self.value
    }
}

pub fn create_circular_list<T>(values: Vec<T>) -> Option<Rc<CyclicNode<T>>> {
    if values.is_empty() {
        return None;
    }
    
    let nodes: Vec<Rc<CyclicNode<T>>> = values
        .into_iter()
        .map(|v| CyclicNode::new(v))
        .collect();
    
    if nodes.len() == 1 {
        CyclicNode::connect(&nodes[0], &nodes[0]);
        return Some(nodes[0].clone());
    }
    
    for i in 0..nodes.len() {
        let next_i = (i + 1) % nodes.len();
        CyclicNode::connect(&nodes[i], &nodes[next_i]);
    }
    
    Some(nodes[0].clone())
}

pub fn reference_count_demo<T>(value: T) -> (usize, usize, usize) {
    let rc1 = Rc::new(value);
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    let weak1 = Rc::downgrade(&rc1);
    
    let strong_count1 = Rc::strong_count(&rc1);
    let weak_count = Rc::weak_count(&rc1);
    let strong_count2 = Rc::strong_count(&rc2);
    
    (strong_count1, weak_count, strong_count2)
}
```

## Explanation

### Rc<T> - Reference Counted Smart Pointer

**Single-Threaded Shared Ownership:**
- `Rc<T>` allows multiple owners of the same data
- Uses reference counting to track number of owners
- Automatically deallocates when reference count reaches zero
- Not thread-safe - use `Arc<T>` for multi-threading

**Key Methods:**
```rust
let rc1 = Rc::new(value);           // Create new Rc
let rc2 = Rc::clone(&rc1);          // Increment reference count (cheap)
let count = Rc::strong_count(&rc1); // Get current reference count
```

### Interior Mutability with Rc<T>

**RefCell for Mutation:**
```rust
children: std::cell::RefCell<Vec<Rc<GraphNode<T>>>>,
parent: std::cell::RefCell<Option<Weak<GraphNode<T>>>>,
```

**Why RefCell is Needed:**
- `Rc<T>` provides shared ownership but is immutable
- `RefCell<T>` provides interior mutability with runtime borrow checking
- Allows mutation of data inside `Rc<T>` through `borrow_mut()`

**Pattern Usage:**
```rust
parent.children.borrow_mut().push(child.clone());  // Mutable borrow
self.children.borrow().len()                       // Immutable borrow
```

### Weak<T> - Breaking Reference Cycles

**Preventing Memory Leaks:**
```rust
*child.parent.borrow_mut() = Some(Rc::downgrade(parent)); // Create weak reference
```

**Key Differences:**
- `Weak<T>` doesn't contribute to reference count
- Must be upgraded to `Rc<T>` before use: `weak.upgrade()`
- Returns `Option<Rc<T>>` - None if value has been dropped
- Prevents circular references that would cause memory leaks

**Circular Reference Problem:**
Without weak references, parent→child→parent cycles would never have their reference count reach zero.

### Arc<T> - Thread-Safe Reference Counting

**Multi-Threading Support:**
```rust
pub fn shared_data_across_threads(data: Vec<String>) -> Vec<usize> {
    let shared_data = Arc::new(data);  // Thread-safe reference counting
    
    for _ in 0..3 {
        let data_clone = Arc::clone(&shared_data);  // Safe to share across threads
        let handle = std::thread::spawn(move || {
            // Each thread has its own Arc clone
        });
    }
}
```

**Performance Considerations:**
- `Arc<T>` uses atomic operations for reference counting
- Slightly more expensive than `Rc<T>` due to thread safety
- Necessary when sharing data across thread boundaries

### Shared Cache Pattern

**Multiple Owners of Cached Data:**
```rust
pub fn insert(&mut self, key: K, value: V) -> Rc<V> {
    let rc_value = Rc::new(value);
    self.data.insert(key, rc_value.clone());  // Cache holds one reference
    rc_value                                   // Caller gets another reference
}
```

**Benefits:**
- Multiple consumers can hold references to same cached data
- Data stays alive as long as any consumer holds a reference
- Efficient sharing without copying large data structures

### Memory Management Patterns

**Reference Count Tracking:**
```rust
let strong_count = Rc::strong_count(&rc);  // Count of Rc<T> instances
let weak_count = Rc::weak_count(&rc);      // Count of Weak<T> instances
```

**Cleanup Behavior:**
- Strong references keep data alive
- When strong count reaches 0, data is dropped
- Weak references don't prevent cleanup
- Weak references become invalid when data is dropped

### Common Use Cases

**Tree Structures with Parent References:**
- Children hold `Rc<Node>` to parents
- Parents hold `Weak<Node>` to children
- Prevents cycles while allowing navigation

**Caching and Sharing:**
- Multiple components share expensive-to-compute data
- Reference counting ensures automatic cleanup
- No need for explicit lifetime management

**Observer Patterns:**
- Observers hold `Weak<Subject>` references
- Subject can be dropped without notifying all observers
- Observers check if subject still exists before use

### Performance and Trade-offs

**Memory Overhead:**
- Reference counting adds memory overhead (counter storage)
- Each clone increments/decrements counter
- Weak references also require additional bookkeeping

**Runtime Cost:**
- Reference counting operations are not free
- Atomic operations in `Arc<T>` have additional cost
- Consider alternatives like borrowed references when possible

**Best Practices:**
1. Use `Rc<T>` for single-threaded shared ownership
2. Use `Arc<T>` only when crossing thread boundaries
3. Use `Weak<T>` to break potential cycles
4. Combine with `RefCell<T>` for interior mutability
5. Profile memory usage for large-scale applications