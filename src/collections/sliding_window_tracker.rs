// cargo test sliding_window_tracker

/// Implement a sliding window minimum/maximum tracker using BTreeMap.
/// Efficiently maintain min/max in a sliding window of size k.
struct SlidingWindowTracker {
    window: std::collections::BTreeMap<i32, usize>, // value -> count
    k: usize,
    current_window: std::collections::VecDeque<i32>,
}

impl SlidingWindowTracker {
    fn new(k: usize) -> Self {
        Self {
            window: std::collections::BTreeMap::new(),
            k: k,
            current_window: std::collections::VecDeque::new(),
        }
    }

    fn add(&mut self, value: i32) {
        todo!()
    }

    fn get_min(&self) -> Option<i32> {
        todo!()
    }

    fn get_max(&self) -> Option<i32> {
        todo!()
    }

    fn window_size(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window_tracker() {
        let mut tracker = SlidingWindowTracker::new(3);

        // Initially empty
        assert_eq!(tracker.get_min(), None);
        assert_eq!(tracker.get_max(), None);

        tracker.add(5);
        assert_eq!(tracker.get_min(), Some(5));
        assert_eq!(tracker.get_max(), Some(5));
        assert_eq!(tracker.window_size(), 1);

        tracker.add(2);
        tracker.add(8);
        assert_eq!(tracker.get_min(), Some(2));
        assert_eq!(tracker.get_max(), Some(8));
        assert_eq!(tracker.window_size(), 3);

        // Adding 4th element should evict first element (5)
        tracker.add(1);
        assert_eq!(tracker.get_min(), Some(1)); // min of [2, 8, 1]
        assert_eq!(tracker.get_max(), Some(8)); // max of [2, 8, 1]
        assert_eq!(tracker.window_size(), 3);

        tracker.add(10);
        assert_eq!(tracker.get_min(), Some(1)); // min of [8, 1, 10]
        assert_eq!(tracker.get_max(), Some(10)); // max of [8, 1, 10]
    }
}
