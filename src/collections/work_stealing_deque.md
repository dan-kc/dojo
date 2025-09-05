# Work-Stealing Deque Solution

## Implementation

```rust
pub struct WorkStealingDeque<T> {
    deque: std::collections::VecDeque<T>,
}

impl<T> WorkStealingDeque<T> {
    pub fn new() -> Self {
        WorkStealingDeque {
            deque: std::collections::VecDeque::new(),
        }
    }

    pub fn push_task(&mut self, task: T) {
        // Owner pushes to back (own end)
        self.deque.push_back(task);
    }

    pub fn pop_task(&mut self) -> Option<T> {
        // Owner pops from back (LIFO - Last In, First Out)
        self.deque.pop_back()
    }

    pub fn steal_task(&mut self) -> Option<T> {
        // Thief steals from front (FIFO - First In, First Out)
        self.deque.pop_front()
    }

    pub fn len(&self) -> usize {
        self.deque.len()
    }

    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}
```

## Explanation

This solution implements a work-stealing deque pattern using VecDeque:

1. **Dual access pattern**: Owner uses LIFO (back), thieves use FIFO (front)
2. **Load balancing**: Workers can steal tasks from busy workers
3. **Cache efficiency**: Owner accesses most recent tasks (better locality)
4. **Work distribution**: Thieves get older tasks, reducing contention
5. **Simple implementation**: VecDeque provides efficient double-ended operations

The pattern optimizes for both cache locality and work distribution.

## Key Learning Points

- **Work-stealing algorithm**: Load balancing technique for parallel systems
- **LIFO vs FIFO access**: Different patterns for owners vs thieves
- **Cache locality optimization**: Recent tasks often share data
- **Contention reduction**: Separate ends reduce synchronization conflicts

## Rust Concepts Demonstrated

- VecDeque for double-ended queue operations
- Different access patterns (push_back/pop_back vs pop_front)
- Generic programming for task types
- Concurrent algorithm patterns (without actual concurrency)
- Load balancing data structure design