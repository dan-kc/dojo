# Sliding Window Min/Max Tracker Solution

## Implementation

```rust
struct SlidingWindowTracker {
    window: std::collections::BTreeMap<i32, usize>, // value -> count
    k: usize,
    current_window: std::collections::VecDeque<i32>,
}

impl SlidingWindowTracker {
    fn new(k: usize) -> Self {
        SlidingWindowTracker {
            window: std::collections::BTreeMap::new(),
            k,
            current_window: std::collections::VecDeque::new(),
        }
    }

    fn add(&mut self, value: i32) {
        // Add new value to window
        self.current_window.push_back(value);
        *self.window.entry(value).or_insert(0) += 1;
        
        // Remove oldest value if window exceeds size k
        if self.current_window.len() > self.k {
            if let Some(old_value) = self.current_window.pop_front() {
                if let Some(count) = self.window.get_mut(&old_value) {
                    *count -= 1;
                    if *count == 0 {
                        self.window.remove(&old_value);
                    }
                }
            }
        }
    }

    fn get_min(&self) -> Option<i32> {
        self.window.keys().next().copied()
    }

    fn get_max(&self) -> Option<i32> {
        self.window.keys().next_back().copied()
    }

    fn window_size(&self) -> usize {
        self.current_window.len()
    }
}
```

## Explanation

This solution combines BTreeMap and VecDeque for efficient sliding window min/max tracking:

1. **BTreeMap for ordering**: Maintains values in sorted order with frequency counts
2. **VecDeque for window**: Tracks insertion order and manages window bounds
3. **Frequency tracking**: Handles duplicate values in the window correctly
4. **Efficient queries**: O(log n) insertions/deletions, O(1) min/max queries
5. **Window management**: Automatically evicts old values when window size exceeded

The BTreeMap provides ordered access while VecDeque manages the sliding window.

## Key Learning Points

- **BTreeMap advantages**: Ordered collection with efficient range queries
- **Frequency counting**: Tracking value occurrences in sliding windows
- **Window boundary management**: Maintaining fixed-size sliding windows
- **Combined data structures**: Using multiple collections for different aspects

## Rust Concepts Demonstrated

- BTreeMap for ordered key-value storage
- VecDeque for sliding window management
- Entry API for frequency counting
- Iterator methods (keys, next, next_back)
- Option handling for empty collections
