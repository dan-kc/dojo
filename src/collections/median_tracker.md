# Median Tracker Solution

## Implementation

```rust
pub struct MedianTracker {
    lower_half: std::collections::BinaryHeap<i32>, // max heap
    upper_half: std::collections::BinaryHeap<std::cmp::Reverse<i32>>, // min heap
}

impl MedianTracker {
    pub fn new() -> Self {
        MedianTracker {
            lower_half: std::collections::BinaryHeap::new(),
            upper_half: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, value: i32) {
        // Decide which heap to add to
        if self.lower_half.is_empty() || value <= *self.lower_half.peek().unwrap_or(&value) {
            self.lower_half.push(value);
        } else {
            self.upper_half.push(std::cmp::Reverse(value));
        }
        
        // Rebalance heaps to maintain the invariant:
        // - lower_half.len() == upper_half.len() OR
        // - lower_half.len() == upper_half.len() + 1
        
        if self.lower_half.len() > self.upper_half.len() + 1 {
            // Move from lower to upper
            if let Some(max_lower) = self.lower_half.pop() {
                self.upper_half.push(std::cmp::Reverse(max_lower));
            }
        } else if self.upper_half.len() > self.lower_half.len() + 1 {
            // Move from upper to lower
            if let Some(std::cmp::Reverse(min_upper)) = self.upper_half.pop() {
                self.lower_half.push(min_upper);
            }
        }
    }

    pub fn get_median(&self) -> Option<f64> {
        match (self.lower_half.len(), self.upper_half.len()) {
            (0, 0) => None,
            (lower_len, upper_len) => {
                if lower_len > upper_len {
                    // Odd number of elements, median is top of lower heap
                    Some(*self.lower_half.peek().unwrap() as f64)
                } else if upper_len > lower_len {
                    // Odd number of elements, median is top of upper heap
                    Some(self.upper_half.peek().unwrap().0 as f64)
                } else {
                    // Even number of elements, median is average of both tops
                    let lower_max = *self.lower_half.peek().unwrap() as f64;
                    let upper_min = self.upper_half.peek().unwrap().0 as f64;
                    Some((lower_max + upper_min) / 2.0)
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.lower_half.len() + self.upper_half.len()
    }
}
```

## Explanation

This solution implements a streaming median tracker using two heaps:

1. **Two-heap structure**: Lower half (max-heap) and upper half (min-heap)
2. **Insertion logic**: Add to appropriate heap based on value comparison
3. **Heap balancing**: Maintain size invariant (differ by at most 1)
4. **Median calculation**: Use heap tops based on total element count
5. **Efficient operations**: O(log n) insertions, O(1) median queries

The algorithm maintains the median by keeping elements balanced between two heaps.

## Key Learning Points

- **Two-heap median pattern**: Classic streaming median algorithm
- **Heap balancing**: Maintaining size constraints for correct median
- **Min-heap simulation**: Using Reverse wrapper for min-heap behavior
- **Stream processing**: Handling continuous data input efficiently

## Rust Concepts Demonstrated

- BinaryHeap for max-heap operations
- `std::cmp::Reverse` for min-heap behavior
- Pattern matching on heap sizes
- Option handling for empty collections
- Efficient streaming algorithms