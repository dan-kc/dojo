// Median Tracker Practice
//
// Learning Objectives:
// - Implement a median tracker using two heaps (BinaryHeap)
// - Efficiently maintain running median as elements are added
// - Practice with max-heap and min-heap patterns using Reverse wrapper
// - Understand heap balancing for streaming median calculation
//
// Run with: cargo test --bin median_tracker

/// Implement a median tracker using two heaps (BinaryHeap).
/// Efficiently maintain running median as elements are added.
pub struct MedianTracker {
    lower_half: std::collections::BinaryHeap<i32>, // max heap
    upper_half: std::collections::BinaryHeap<std::cmp::Reverse<i32>>, // min heap
}

impl MedianTracker {
    pub fn new() -> Self {
        todo!("Create new median tracker")
    }

    pub fn add(&mut self, value: i32) {
        todo!("Add value while maintaining median property")
    }

    pub fn get_median(&self) -> Option<f64> {
        todo!("Get current median")
    }

    pub fn len(&self) -> usize {
        self.lower_half.len() + self.upper_half.len()
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
        
        assert_eq!(tracker.len(), 5);
    }

    #[test]
    fn test_single_element() {
        let mut tracker = MedianTracker::new();
        tracker.add(42);
        assert_eq!(tracker.get_median(), Some(42.0));
    }
}