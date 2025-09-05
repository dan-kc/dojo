# Circular Buffer Solution

## Implementation

```rust
pub struct CircularBuffer<T> {
    buffer: std::collections::VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: std::collections::VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        // Add to front (newest)
        self.buffer.push_front(item);
        
        // Remove oldest if over capacity
        if self.buffer.len() > self.capacity {
            self.buffer.pop_back();
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // Index 0 is newest (front)
        self.buffer.get(index)
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        // Return iterator from newest to oldest
        self.buffer.iter()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.capacity
    }
}
```

## Explanation

This solution implements a circular buffer with fixed capacity:

1. **Fixed capacity**: Buffer never exceeds specified size
2. **FIFO eviction**: Oldest element removed when full
3. **Newest-first ordering**: Index 0 is most recent element
4. **VecDeque backing**: Efficient O(1) operations at both ends
5. **Automatic overflow**: Seamlessly handles capacity overflow

## Key Learning Points

- **Circular semantics**: Overwriting oldest data when full
- **VecDeque efficiency**: O(1) push_front and pop_back operations
- **Index convention**: 0 = newest, higher = older
- **Bounded collections**: Maintaining fixed memory footprint

## Rust Concepts Demonstrated

- VecDeque for double-ended operations
- Generic type parameters for buffer contents
- Iterator lifetime parameters
- Capacity management patterns
- Option type for safe indexing