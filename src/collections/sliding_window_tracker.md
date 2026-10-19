# collections/sliding_window_tracker

Reference implementation. Read after attempting the exercise.

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
        self.current_window.push_back(value);
        *self.window.entry(value).or_insert(0) += 1;

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
