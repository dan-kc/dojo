# Hierarchical Locking System

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct HierarchicalLockManager {
    resources: Vec<Arc<Mutex<(usize, String)>>>, // (level, data)
}

impl HierarchicalLockManager {
    pub fn new(resources: Vec<(usize, String)>) -> Self {
        let resources = resources
            .into_iter()
            .map(|(level, data)| Arc::new(Mutex::new((level, data))))
            .collect();
        Self { resources }
    }

    pub fn acquire_hierarchical(&self, resource_ids: Vec<usize>) -> Option<Vec<String>> {
        // First, get the hierarchy levels for sorting
        let mut resource_levels = Vec::new();
        
        for &resource_id in &resource_ids {
            if resource_id >= self.resources.len() {
                return None;
            }
            
            // Get the level for sorting (peek at the level without locking)
            let lock = self.resources[resource_id].lock().unwrap();
            let level = lock.0;
            drop(lock);
            
            resource_levels.push((resource_id, level));
        }
        
        // Sort by hierarchy level (descending - highest level first)
        resource_levels.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Now acquire locks in hierarchical order
        let mut acquired_locks = Vec::new();
        let mut acquired_data = Vec::new();
        
        for &(resource_id, _level) in &resource_levels {
            match self.resources[resource_id].try_lock() {
                Ok(lock) => {
                    acquired_data.push(lock.1.clone());
                    acquired_locks.push(lock);
                }
                Err(_) => {
                    // Failed to acquire a resource in hierarchical order
                    return None;
                }
            }
        }
        
        Some(acquired_data)
    }
}

pub fn deadlock_detector<F>(operation: F, timeout: Duration) -> Result<(), &'static str>
where
    F: FnOnce() + Send + 'static,
{
    let (sender, receiver) = std::sync::mpsc::channel();
    
    // Spawn the operation in a separate thread
    thread::spawn(move || {
        operation();
        let _ = sender.send(()); // Signal completion
    });
    
    // Wait for completion or timeout
    match receiver.recv_timeout(timeout) {
        Ok(_) => Ok(()),
        Err(_) => Err("Operation timed out - possible deadlock detected"),
    }
}
```

## Explanation

This solution implements **hierarchical locking** to prevent deadlocks through **resource ordering**:

### Key Concepts Demonstrated:

1. **Hierarchical Resource Ordering**:
   - Resources are assigned hierarchy levels (priorities)
   - Higher-level resources must always be acquired before lower-level ones
   - Creates a strict partial ordering that prevents circular dependencies

2. **Deadlock Prevention Strategy**:
   - By enforcing a consistent acquisition order, circular wait conditions are eliminated
   - No two threads can create a cycle in the resource dependency graph
   - Similar to lock ordering but based on logical resource hierarchy rather than IDs

3. **Level-based Sorting**:
   - Resources are sorted by hierarchy level before acquisition
   - Highest level (most important) resources are acquired first
   - Ensures consistent ordering across all threads

4. **Timeout-based Deadlock Detection**:
   - Monitors operation execution time using channels and timeouts
   - Can detect potential deadlocks by timing out long-running operations
   - Provides a safety net for detecting unexpected blocking

### How Hierarchical Locking Works:

```
Resource Hierarchy Example:
Level 3: Critical System Resource
Level 2: Database Connection  
Level 1: User Session Data

Acquisition Order: Level 3 → Level 2 → Level 1
```

### Deadlock Prevention Mechanism:

1. **Consistent Ordering**: All threads acquire resources in the same hierarchy order
2. **No Circular Waits**: Higher-level resources are always acquired first
3. **Transitive Property**: If A has higher level than B, and B higher than C, then A > B > C

### Deadlock Detector Implementation:

The deadlock detector uses:
- **Channel communication** to signal operation completion
- **Timeout mechanism** to detect hung operations  
- **Separate thread execution** to isolate potentially blocking operations

### Use Cases:

Hierarchical locking is particularly useful in:
- **Database systems**: Table locks before row locks
- **Operating systems**: Kernel resources before user resources  
- **Distributed systems**: Global locks before local locks
- **Resource management**: Critical resources before optional ones

### Advantages:

1. **Deadlock Prevention**: Mathematically guarantees no circular waits
2. **Priority Enforcement**: Important resources are acquired first
3. **Deterministic Behavior**: Consistent acquisition order across threads
4. **Scalable**: Works with arbitrary numbers of resources and threads

This approach demonstrates how logical resource hierarchy can provide both deadlock prevention and priority-based resource allocation in concurrent systems.