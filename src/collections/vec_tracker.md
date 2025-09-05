# Vec Performance Tracker Solution

## Implementation

```rust
pub struct VecTracker<T> {
    vec: Vec<T>,
    capacity_changes: usize,
    operations_count: usize,
}

impl<T> VecTracker<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            capacity_changes: 0,
            operations_count: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            capacity_changes: 0,
            operations_count: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        let old_capacity = self.vec.capacity();
        self.vec.push(item);
        let new_capacity = self.vec.capacity();
        
        if new_capacity > old_capacity {
            self.capacity_changes += 1;
        }
        
        self.operations_count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.operations_count += 1;
        self.vec.pop()
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let old_capacity = self.vec.capacity();
        self.vec.extend(iter);
        let new_capacity = self.vec.capacity();
        
        if new_capacity > old_capacity {
            self.capacity_changes += 1;
        }
        
        self.operations_count += 1;
    }

    pub fn capacity_changes(&self) -> usize {
        self.capacity_changes
    }

    pub fn operations_count(&self) -> usize {
        self.operations_count
    }

    pub fn into_vec(self) -> Vec<T> {
        self.vec
    }
}
```

## Explanation

This solution implements a Vec wrapper that tracks performance metrics:

1. **Wrapper struct**: Contains the original Vec and tracking counters
2. **Capacity monitoring**: Compares capacity before and after operations to detect reallocations
3. **Operation counting**: Increments counter for each method call
4. **Transparent interface**: Provides the same methods as Vec with added tracking

## Key Learning Points

- **Wrapper patterns**: Encapsulating existing types to add functionality
- **Capacity tracking**: Understanding Vec's reallocation behavior
- **Performance monitoring**: Measuring operations and memory changes
- **Generic implementation**: Works with any type T stored in the Vec

## Design Insights

- **Pre-allocation benefit**: `with_capacity()` reduces reallocation frequency
- **Growth patterns**: Vec typically doubles capacity when reallocating
- **Operation granularity**: Each high-level operation counts as one, regardless of internal complexity
- **Memory efficiency**: The wrapper adds minimal overhead (two usize fields)

## Rust Concepts Demonstrated

- Wrapper type patterns and composition
- Vec capacity management and reallocation behavior
- Generic struct implementation with associated methods
- Performance monitoring and instrumentation
- Memory allocation patterns in Rust collections
- Ownership transfer through `into_vec()` consumption