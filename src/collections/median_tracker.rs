// cargo test median_tracker

/// Implement a median tracker using two heaps (BinaryHeap).
/// Efficiently maintain running median as elements are added.
pub struct MedianTracker {
    upper_half: std::collections::BinaryHeap<std::cmp::Reverse<i32>>, // Larger or equal
    lower_half: std::collections::BinaryHeap<i32>,
}

impl MedianTracker {
    pub fn new() -> Self {
        Self {
            upper_half: std::collections::BinaryHeap::new(),
            lower_half: std::collections::BinaryHeap::new(),
        }
    }

    pub fn add(&mut self, value: i32) {
        todo!()
    }

    pub fn get_median(&self) -> Option<f64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_tracker() {
        let mut tracker = MedianTracker::new();

        tracker.add(1);
        assert_eq!(tracker.get_median(), Some(1.0));

        tracker.add(2);
        assert_eq!(tracker.get_median(), Some(1.5)); // (1+2)/2

        tracker.add(3);
        assert_eq!(tracker.get_median(), Some(2.0)); // middle of [1,2,3]

        tracker.add(4);
        assert_eq!(tracker.get_median(), Some(2.5)); // (2+3)/2 for [1,2,3,4]

        tracker.add(5);
        assert_eq!(tracker.get_median(), Some(3.0)); // middle of [1,2,3,4,5]
    }

    #[test]
    fn test_single_element() {
        let mut tracker = MedianTracker::new();
        tracker.add(42);
        assert_eq!(tracker.get_median(), Some(42.0));
    }
}
