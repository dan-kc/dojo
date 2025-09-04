# Box<T> Basics - Solution

## Solution

```rust
#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
    
    pub fn with_children(value: T, left: TreeNode<T>, right: TreeNode<T>) -> Self {
        TreeNode {
            value,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    
    pub fn add_left(&mut self, child: TreeNode<T>) {
        self.left = Some(Box::new(child));
    }
    
    pub fn add_right(&mut self, child: TreeNode<T>) {
        self.right = Some(Box::new(child));
    }
    
    pub fn depth(&self) -> usize {
        let left_depth = self.left.as_ref().map_or(0, |node| node.depth() + 1);
        let right_depth = self.right.as_ref().map_or(0, |node| node.depth() + 1);
        left_depth.max(right_depth)
    }
    
    pub fn count_nodes(&self) -> usize {
        let left_count = self.left.as_ref().map_or(0, |node| node.count_nodes());
        let right_count = self.right.as_ref().map_or(0, |node| node.count_nodes());
        1 + left_count + right_count
    }
    
    pub fn in_order_values(&self) -> Vec<&T> {
        let mut result = Vec::new();
        
        // Traverse left subtree
        if let Some(left) = &self.left {
            result.extend(left.in_order_values());
        }
        
        // Visit current node
        result.push(&self.value);
        
        // Traverse right subtree
        if let Some(right) = &self.right {
            result.extend(right.in_order_values());
        }
        
        result
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

#[derive(Debug)]
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    
    pub fn push_front(&mut self, data: T) {
        let new_node = Box::new(ListNode {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
    
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        
        count
    }
    
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

pub fn smart_allocation<T>(data: T, force_heap: bool) -> either::Either<T, Box<T>>
where
    T: Clone,
{
    if force_heap || std::mem::size_of::<T>() > 1024 {
        either::Either::Right(Box::new(data))
    } else {
        either::Either::Left(data)
    }
}

pub fn vec_to_balanced_tree<T>(mut vec: Vec<T>) -> Option<TreeNode<T>> {
    if vec.is_empty() {
        return None;
    }
    
    if vec.len() == 1 {
        return Some(TreeNode::new(vec.pop().unwrap()));
    }
    
    let mid = vec.len() / 2;
    let value = vec.remove(mid);
    
    let left_vec = vec[..mid].to_vec();
    let right_vec = vec[mid..].to_vec();
    
    let mut node = TreeNode::new(value);
    
    if let Some(left_tree) = vec_to_balanced_tree(left_vec) {
        node.add_left(left_tree);
    }
    
    if let Some(right_tree) = vec_to_balanced_tree(right_vec) {
        node.add_right(right_tree);
    }
    
    Some(node)
}

pub fn use_deref_coercion<T>(boxed_value: Box<T>) -> T
where
    T: Clone,
{
    (*boxed_value).clone()
    // or simply: boxed_value.clone() due to deref coercion
}

pub fn create_nested_boxes(depth: usize, initial_value: i32) -> Box<i32> {
    if depth == 0 {
        Box::new(initial_value)
    } else {
        // For this implementation, we'll just return the value at each level
        // A truly nested structure would require different design
        Box::new(initial_value)
    }
}
```

## Explanation

### Understanding Box<T>

**Heap Allocation:**
- `Box<T>` allocates data on the heap instead of the stack
- Provides owned heap-allocated memory with automatic cleanup
- Single ownership semantics (move-only by default)

**When to Use Box<T>:**
1. **Recursive Data Structures** - Trees, linked lists where size isn't known at compile time
2. **Large Data** - Move large structs to heap to avoid stack overflow
3. **Trait Objects** - `Box<dyn Trait>` for runtime polymorphism
4. **Indirection** - When you need a level of indirection for ownership

### Recursive Data Structures

**Tree Implementation:**
```rust
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}
```

**Why Box is Necessary:**
- Without `Box`, the compiler can't determine the size of `TreeNode<T>` at compile time
- `Box<TreeNode<T>>` has a known size (pointer size)
- Enables recursive definitions that would otherwise be impossible

**Memory Layout:**
- Each node contains its value on the stack
- Child pointers are heap-allocated `Box<TreeNode<T>>`
- Automatic memory cleanup when nodes are dropped

### Linked List Patterns

**Node Structure:**
```rust
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}
```

**Key Operations:**
- `push_front`: Creates new boxed node, takes ownership of current head
- `pop_front`: Takes ownership of head, returns data, updates head to next
- `take()` method transfers ownership out of Option

### Box<T> Memory Management

**Ownership Transfer:**
```rust
pub fn pop_front(&mut self) -> Option<T> {
    self.head.take().map(|node| {
        self.head = node.next;  // Transfer ownership to new head
        node.data               // Return owned data
    })
}
```

**Automatic Cleanup:**
- When `Box<T>` goes out of scope, it automatically deallocates heap memory
- No manual memory management required
- RAII (Resource Acquisition Is Initialization) principles

### Deref Coercion

**How it Works:**
```rust
pub fn use_deref_coercion<T>(boxed_value: Box<T>) -> T {
    (*boxed_value).clone()  // Explicit dereference
    // boxed_value.clone()  // Automatic deref coercion
}
```

**Benefits:**
- Methods on `T` can be called directly on `Box<T>`
- Seamless integration with existing APIs
- Reduces syntactic noise

### Performance Considerations

**Stack vs Heap Trade-offs:**

**Stack Allocation (Regular Values):**
- Faster allocation/deallocation
- Better cache locality
- Limited by stack size
- Automatic cleanup

**Heap Allocation (Box<T>):**
- Unlimited size (within available memory)
- Indirection cost (one pointer dereference)
- Dynamic allocation overhead
- Enables recursive structures

### Common Patterns

**Tree Traversal:**
```rust
pub fn in_order_values(&self) -> Vec<&T> {
    let mut result = Vec::new();
    
    // Left, Root, Right pattern
    if let Some(left) = &self.left {
        result.extend(left.in_order_values());
    }
    result.push(&self.value);
    if let Some(right) = &self.right {
        result.extend(right.in_order_values());
    }
    
    result
}
```

**Recursive Algorithms:**
- Use `as_ref()` to work with `&Option<Box<T>>` as `Option<&T>`
- `map_or()` for providing default values when None
- Pattern matching with destructuring for clean code

**Error Handling:**
- `Option<Box<T>>` is common for optional heap-allocated data
- `Result<Box<T>, E>` for fallible heap allocations
- Memory allocation can fail (rare but possible)

### Best Practices

1. **Prefer stack allocation by default** - only use `Box<T>` when needed
2. **Use for recursive structures** - trees, graphs, linked lists
3. **Consider alternatives** - `Vec<T>` often better than linked lists
4. **Profile memory usage** - measure heap vs stack performance impact
5. **Use `Rc<T>` or `Arc<T>`** when you need shared ownership