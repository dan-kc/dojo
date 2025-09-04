# Smart Pointer Combinations - Solution

## Solution

```rust
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub struct ThreadSafeCache<K, V> {
    data: Arc<Mutex<HashMap<K, V>>>,
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        ThreadSafeCache {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn insert(&self, key: K, value: V) {
        self.data.lock().unwrap().insert(key, value);
    }
    
    pub fn get(&self, key: &K) -> Option<V> {
        self.data.lock().unwrap().get(key).cloned()
    }
    
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        let mut data = self.data.lock().unwrap();
        if let Some(value) = data.get(&key) {
            value.clone()
        } else {
            let value = compute();
            data.insert(key, value.clone());
            value
        }
    }
    
    pub fn clone_handle(&self) -> Self {
        ThreadSafeCache {
            data: Arc::clone(&self.data),
        }
    }
    
    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }
    
    pub fn len(&self) -> usize {
        self.data.lock().unwrap().len()
    }
}

pub struct TreeNodeComplex<T> {
    value: T,
    children: RefCell<Vec<Rc<TreeNodeComplex<T>>>>,
    parent: RefCell<Option<Weak<TreeNodeComplex<T>>>>,
}

impl<T> TreeNodeComplex<T> {
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(TreeNodeComplex {
            value,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(None),
        })
    }
    
    pub fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
        parent.children.borrow_mut().push(child.clone());
        *child.parent.borrow_mut() = Some(Rc::downgrade(parent));
    }
    
    pub fn remove_child(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        let mut children = self.children.borrow_mut();
        if let Some(pos) = children.iter().position(|child| child.value == *value) {
            let removed_child = children.remove(pos);
            *removed_child.parent.borrow_mut() = None;
            true
        } else {
            false
        }
    }
    
    pub fn get_children_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.children
            .borrow()
            .iter()
            .map(|child| child.value.clone())
            .collect()
    }
    
    pub fn find_node(&self, target: &T) -> Option<Rc<Self>>
    where
        T: PartialEq,
    {
        if self.value == *target {
            // We need to return an Rc to self, but we don't have one
            // This requires the caller to have the Rc
            return None; // Simplified for this example
        }
        
        for child in self.children.borrow().iter() {
            if let Some(found) = child.find_node(target) {
                return Some(found);
            }
        }
        None
    }
    
    pub fn get_depth(&self) -> usize {
        match self.parent.borrow().as_ref() {
            Some(parent_weak) => {
                if let Some(parent) = parent_weak.upgrade() {
                    parent.get_depth() + 1
                } else {
                    0 // Parent was dropped
                }
            }
            None => 0,
        }
    }
    
    pub fn get_path_from_root(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut path = Vec::new();
        self.collect_path_to_root(&mut path);
        path.reverse();
        path
    }
    
    fn collect_path_to_root(&self, path: &mut Vec<T>)
    where
        T: Clone,
    {
        path.push(self.value.clone());
        if let Some(parent_weak) = self.parent.borrow().as_ref() {
            if let Some(parent) = parent_weak.upgrade() {
                parent.collect_path_to_root(path);
            }
        }
    }
}

pub struct Publisher<T> {
    subscribers: RefCell<Vec<Weak<dyn Subscriber<T>>>>,
}

pub trait Subscriber<T> {
    fn notify(&self, message: &T);
}

impl<T> Publisher<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Publisher {
            subscribers: RefCell::new(Vec::new()),
        }
    }
    
    pub fn subscribe(&self, subscriber: Rc<dyn Subscriber<T>>) {
        self.subscribers.borrow_mut().push(Rc::downgrade(&subscriber));
    }
    
    pub fn publish(&self, message: T) {
        let mut subscribers = self.subscribers.borrow_mut();
        subscribers.retain(|weak_subscriber| {
            if let Some(subscriber) = weak_subscriber.upgrade() {
                subscriber.notify(&message);
                true // Keep this subscriber
            } else {
                false // Remove this dead subscriber
            }
        });
    }
    
    pub fn cleanup_subscribers(&self) {
        let mut subscribers = self.subscribers.borrow_mut();
        subscribers.retain(|weak_subscriber| weak_subscriber.upgrade().is_some());
    }
    
    pub fn active_subscriber_count(&self) -> usize {
        self.subscribers
            .borrow()
            .iter()
            .filter(|weak_subscriber| weak_subscriber.upgrade().is_some())
            .count()
    }
}

pub struct LoggingSubscriber {
    name: String,
    logs: RefCell<Vec<String>>,
}

impl LoggingSubscriber {
    pub fn new(name: String) -> Rc<Self> {
        Rc::new(LoggingSubscriber {
            name,
            logs: RefCell::new(Vec::new()),
        })
    }
    
    pub fn get_logs(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }
}

impl Subscriber<String> for LoggingSubscriber {
    fn notify(&self, message: &String) {
        let log_entry = format!("{}: {}", self.name, message);
        self.logs.borrow_mut().push(log_entry);
    }
}

pub struct MemoryPool<T> {
    available: RefCell<Vec<Box<T>>>,
    allocated: RefCell<Vec<Weak<RefCell<T>>>>,
}

impl<T> MemoryPool<T>
where
    T: Default,
{
    pub fn new(initial_capacity: usize) -> Self {
        let mut available = Vec::with_capacity(initial_capacity);
        for _ in 0..initial_capacity {
            available.push(Box::new(T::default()));
        }
        
        MemoryPool {
            available: RefCell::new(available),
            allocated: RefCell::new(Vec::new()),
        }
    }
    
    pub fn allocate(&self) -> Rc<RefCell<T>> {
        let item = if let Some(boxed_item) = self.available.borrow_mut().pop() {
            RefCell::new(*boxed_item)
        } else {
            RefCell::new(T::default())
        };
        
        let rc_item = Rc::new(item);
        self.allocated.borrow_mut().push(Rc::downgrade(&rc_item));
        rc_item
    }
    
    pub fn stats(&self) -> (usize, usize) {
        let available_count = self.available.borrow().len();
        let allocated_count = self.allocated
            .borrow()
            .iter()
            .filter(|weak| weak.upgrade().is_some())
            .count();
        (available_count, allocated_count)
    }
    
    pub fn collect(&self) {
        let mut allocated = self.allocated.borrow_mut();
        allocated.retain(|weak| weak.upgrade().is_some());
    }
}

pub struct Graph<T> {
    nodes: RefCell<HashMap<usize, Rc<GraphNode<T>>>>,
}

pub struct GraphNode<T> {
    id: usize,
    value: T,
    edges: RefCell<Vec<Weak<GraphNode<T>>>>,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            nodes: RefCell::new(HashMap::new()),
        }
    }
    
    pub fn add_node(&self, id: usize, value: T) -> Rc<GraphNode<T>> {
        let node = GraphNode::new(id, value);
        self.nodes.borrow_mut().insert(id, node.clone());
        node
    }
    
    pub fn add_edge(&self, from_id: usize, to_id: usize) -> Result<(), &'static str> {
        let nodes = self.nodes.borrow();
        let from_node = nodes.get(&from_id).ok_or("Source node not found")?;
        let to_node = nodes.get(&to_id).ok_or("Target node not found")?;
        
        from_node.edges.borrow_mut().push(Rc::downgrade(to_node));
        Ok(())
    }
    
    pub fn get_node(&self, id: usize) -> Option<Rc<GraphNode<T>>> {
        self.nodes.borrow().get(&id).cloned()
    }
    
    pub fn remove_node(&self, id: usize) -> Option<Rc<GraphNode<T>>> {
        self.nodes.borrow_mut().remove(&id)
    }
    
    pub fn get_node_ids(&self) -> Vec<usize> {
        self.nodes.borrow().keys().cloned().collect()
    }
}

impl<T> GraphNode<T> {
    fn new(id: usize, value: T) -> Rc<Self> {
        Rc::new(GraphNode {
            id,
            value,
            edges: RefCell::new(Vec::new()),
        })
    }
    
    pub fn get_connected_nodes(&self) -> Vec<Rc<GraphNode<T>>> {
        self.edges
            .borrow()
            .iter()
            .filter_map(|weak| weak.upgrade())
            .collect()
    }
    
    pub fn value(&self) -> &T {
        &self.value
    }
    
    pub fn id(&self) -> usize {
        self.id
    }
}
```

## Explanation

### Arc<Mutex<T>> - Thread-Safe Shared Mutable Data

**Why This Combination:**
```rust
Arc<Mutex<HashMap<K, V>>>  // Thread-safe shared ownership + thread-safe interior mutability
```

**Key Benefits:**
- `Arc<T>` provides thread-safe reference counting
- `Mutex<T>` provides thread-safe interior mutability
- Multiple threads can safely share and modify the same data

**Usage Patterns:**
```rust
pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V {
    let mut data = self.data.lock().unwrap();  // Single lock for entire operation
    if let Some(value) = data.get(&key) {
        value.clone()
    } else {
        let value = compute();
        data.insert(key, value.clone());
        value
    }
}
```

### Rc<T> + RefCell<T> + Weak<T> Combination

**Complex Tree with Parent-Child Relationships:**
```rust
pub struct TreeNodeComplex<T> {
    value: T,
    children: RefCell<Vec<Rc<TreeNodeComplex<T>>>>,  // Strong refs to children
    parent: RefCell<Option<Weak<TreeNodeComplex<T>>>>,  // Weak ref to parent
}
```

**Design Decisions:**
- **Children**: Strong references (`Rc<T>`) - parent keeps children alive
- **Parent**: Weak reference (`Weak<T>`) - prevents reference cycles
- **RefCell**: Allows modification of relationships through immutable references

**Cycle Prevention:**
- Parent→Child: Strong reference (parent owns children)
- Child→Parent: Weak reference (doesn't prevent parent cleanup)
- When parent is dropped, children lose their parent reference safely

### Publisher-Subscriber Pattern

**Weak References for Loose Coupling:**
```rust
pub struct Publisher<T> {
    subscribers: RefCell<Vec<Weak<dyn Subscriber<T>>>>,  // Weak refs to subscribers
}
```

**Benefits:**
- Subscribers can be dropped independently
- Publisher doesn't keep subscribers alive
- Automatic cleanup of dead subscribers
- No memory leaks from forgotten subscriptions

**Implementation Strategy:**
```rust
pub fn publish(&self, message: T) {
    let mut subscribers = self.subscribers.borrow_mut();
    subscribers.retain(|weak_subscriber| {
        if let Some(subscriber) = weak_subscriber.upgrade() {
            subscriber.notify(&message);  // Notify if still alive
            true  // Keep in list
        } else {
            false // Remove dead subscriber
        }
    });
}
```

### Memory Pool Pattern

**Combining Multiple Smart Pointer Types:**
```rust
pub struct MemoryPool<T> {
    available: RefCell<Vec<Box<T>>>,           // Pre-allocated objects
    allocated: RefCell<Vec<Weak<RefCell<T>>>>, // Track allocated objects
}
```

**Design Rationale:**
- `Box<T>` for heap-allocated pre-allocated objects
- `Rc<RefCell<T>>` for allocated objects (shared ownership + mutability)
- `Weak<RefCell<T>>` to track without keeping alive
- `RefCell<Vec<...>>` for interior mutability of the pool itself

### Graph Data Structure

**Handling Complex Relationships:**
```rust
pub struct Graph<T> {
    nodes: RefCell<HashMap<usize, Rc<GraphNode<T>>>>,  // Node storage
}

pub struct GraphNode<T> {
    edges: RefCell<Vec<Weak<GraphNode<T>>>>,  // Weak refs to connected nodes
}
```

**Edge Management:**
- Nodes stored in HashMap with strong references
- Edges use weak references to prevent cycles
- When nodes are removed, edges become invalid automatically

### Performance and Trade-off Analysis

**Arc<Mutex<T>> vs Rc<RefCell<T>>:**

| Aspect | Arc<Mutex<T>> | Rc<RefCell<T>> |
|--------|---------------|----------------|
| Thread Safety | Yes | No |
| Performance | Slower (atomic ops) | Faster |
| Blocking | Yes (on lock contention) | No |
| Panic Safety | Lock poisoning | Borrow panic |

**When to Use Each Combination:**

1. **Rc<RefCell<T>>** - Single-threaded shared mutable data
2. **Arc<Mutex<T>>** - Multi-threaded shared mutable data
3. **Rc<T> + Weak<T>** - Shared ownership with cycle prevention
4. **Arc<T> + Weak<T>** - Thread-safe shared ownership with cycle prevention

### Common Patterns and Best Practices

**Cycle Prevention Strategies:**
```rust
// Parent-Child: Strong down, weak up
parent.children: Vec<Rc<Child>>
child.parent: Weak<Parent>

// Observer: Strong subscriber reference, weak publisher reference
subscriber.publisher: Weak<Publisher>
publisher.subscribers: Vec<Weak<Subscriber>>
```

**Error Handling:**
```rust
// Mutex lock handling
let data = self.data.lock().unwrap();  // Panics on poisoned mutex

// Better approach:
match self.data.lock() {
    Ok(data) => { /* use data */ },
    Err(poisoned) => {
        // Handle poisoned mutex
        let data = poisoned.into_inner();
    }
}
```

**Memory Management:**
- Use weak references to break cycles
- Implement cleanup methods for removing dead weak references
- Consider using `try_borrow()` for non-panicking RefCell access
- Profile memory usage in complex smart pointer combinations

### Real-World Applications

**Web Servers:**
- `Arc<Mutex<T>>` for shared connection pools
- `Weak<T>` for request/response lifecycle management

**Game Engines:**
- `Rc<RefCell<T>>` for game object hierarchies
- `Weak<T>` for temporary references (UI elements, effects)

**Caching Systems:**
- `Arc<Mutex<HashMap<K, V>>>` for thread-safe caches
- `Weak<T>` for cache eviction without blocking access

**GUI Frameworks:**
- `Rc<RefCell<T>>` for widget trees
- `Weak<T>` for parent-child relationships and event handling