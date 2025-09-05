# Task Scheduler Solution

## Implementation

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u32,
    pub priority: u32,
    pub description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then by ID for tie-breaking
        self.priority.cmp(&other.priority)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct TaskScheduler {
    heap: std::collections::BinaryHeap<Task>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler {
            heap: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.heap.push(task);
    }

    pub fn get_next_task(&mut self) -> Option<Task> {
        self.heap.pop()
    }

    pub fn peek_next_task(&self) -> Option<&Task> {
        self.heap.peek()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
}
```

## Explanation

This solution implements a priority-based task scheduler using BinaryHeap:

1. **Task ordering**: Higher priority tasks are processed first
2. **Tie-breaking**: Use task ID for consistent ordering when priorities equal
3. **Max-heap behavior**: BinaryHeap naturally provides max-heap for highest priority
4. **Scheduler operations**: Add tasks, get next task, peek without removal
5. **Priority queue pattern**: Classic use case for heap data structure

The scheduler ensures highest priority tasks are always processed first.

## Key Learning Points

- **Priority queue applications**: Task scheduling is classic heap use case
- **Custom Ord implementation**: Defining priority ordering for domain objects
- **Heap operations**: Push, pop, and peek operations for scheduling
- **Tie-breaking strategies**: Handling equal priorities consistently

## Rust Concepts Demonstrated

- BinaryHeap for max-heap operations
- Custom trait implementations (Ord, PartialOrd)
- Struct definition with public fields
- Priority queue patterns in Rust
- Scheduling algorithm design