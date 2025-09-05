// Vec Performance Tracker Practice
//
// Learning objectives:
// - Wrapping Vec to track performance characteristics
// - Understanding Vec capacity management and reallocations
// - Implementing wrapper types with custom tracking
//
// Run with: cargo test vec_tracker

/// Create a custom Vec wrapper that tracks capacity changes and operations.
pub struct VecTracker<T> {
    vec: Vec<T>,
    capacity_changes: usize,
    operations_count: usize,
}

impl<T> VecTracker<T> {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    pub fn with_capacity(capacity: usize) -> Self {
        todo!("Implement with_capacity")
    }

    pub fn push(&mut self, item: T) {
        todo!("Implement tracked push")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("Implement tracked pop")
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!("Implement tracked extend")
    }

    pub fn capacity_changes(&self) -> usize {
        self.capacity_changes
    }

    pub fn operations_count(&self) -> usize {
        self.operations_count
    }

    pub fn into_vec(self) -> Vec<T> {
        self.vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_tracker() {
        let mut tracker = VecTracker::with_capacity(2);
        assert_eq!(tracker.capacity_changes(), 0); // Initial capacity doesn't count
        
        tracker.push(1);
        tracker.push(2);
        assert_eq!(tracker.operations_count(), 2);
        assert_eq!(tracker.capacity_changes(), 0); // No reallocation yet
        
        tracker.push(3); // Should trigger reallocation
        assert_eq!(tracker.capacity_changes(), 1);
        
        tracker.extend(vec![4, 5, 6, 7]); // Might trigger another reallocation
        assert!(tracker.operations_count() >= 3);
        
        let popped = tracker.pop();
        assert_eq!(popped, Some(7));
        
        let final_vec = tracker.into_vec();
        assert_eq!(final_vec, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_performance_characteristics() {
        // Test that with_capacity reduces reallocations
        let mut tracker_with_capacity = VecTracker::with_capacity(1000);
        let mut tracker_without_capacity = VecTracker::new();
        
        for i in 0..1000 {
            tracker_with_capacity.push(i);
            tracker_without_capacity.push(i);
        }
        
        // Pre-allocated version should have fewer capacity changes
        assert!(tracker_with_capacity.capacity_changes() < tracker_without_capacity.capacity_changes());
    }

    #[test]
    fn test_empty_tracker() {
        let mut tracker: VecTracker<i32> = VecTracker::new();
        assert_eq!(tracker.operations_count(), 0);
        assert_eq!(tracker.capacity_changes(), 0);
        
        let popped = tracker.pop();
        assert_eq!(popped, None);
        assert_eq!(tracker.operations_count(), 1);
        
        let vec = tracker.into_vec();
        assert!(vec.is_empty());
    }

    #[test]
    fn test_extend_tracking() {
        let mut tracker: VecTracker<i32> = VecTracker::new();
        tracker.extend(vec![1, 2, 3]);
        assert_eq!(tracker.operations_count(), 1);
        
        tracker.extend(std::iter::empty::<i32>());
        assert_eq!(tracker.operations_count(), 2);
        
        let vec = tracker.into_vec();
        assert_eq!(vec, vec![1, 2, 3]);
    }
}