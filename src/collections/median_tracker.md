# collections/median_tracker

Reference implementation. Read after attempting the exercise.

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
        if self.lower_half.is_empty() || value <= *self.lower_half.peek().unwrap_or(&value) {
            self.lower_half.push(value);
        } else {
            self.upper_half.push(std::cmp::Reverse(value));
        }

        if self.lower_half.len() > self.upper_half.len() + 1 {
            if let Some(max_lower) = self.lower_half.pop() {
                self.upper_half.push(std::cmp::Reverse(max_lower));
            }
        } else if self.upper_half.len() > self.lower_half.len() + 1 {
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
                    Some(*self.lower_half.peek().unwrap() as f64)
                } else if upper_len > lower_len {
                    Some(self.upper_half.peek().unwrap().0 as f64)
                } else {
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
