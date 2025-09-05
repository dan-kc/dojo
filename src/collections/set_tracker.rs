// Set Change Tracker Practice
//
// Learning objectives:
// - Track set membership changes over time
// - Maintain history of additions and removals
// - Implement stateful data structures with HashSet
//
// Run with: cargo test set_tracker

/// Track set membership changes over time.
/// Maintain history of additions and removals to a set.
pub struct SetTracker<T> {
    current_set: std::collections::HashSet<T>,
    additions: Vec<T>,
    removals: Vec<T>,
}

impl<T> SetTracker<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        todo!("Implement new SetTracker")
    }

    pub fn insert(&mut self, item: T) -> bool {
        todo!("Implement tracked insert")
    }

    pub fn remove(&mut self, item: &T) -> bool {
        todo!("Implement tracked remove")
    }

    pub fn contains(&self, item: &T) -> bool {
        self.current_set.contains(item)
    }

    pub fn addition_history(&self) -> &[T] {
        &self.additions
    }

    pub fn removal_history(&self) -> &[T] {
        &self.removals
    }

    pub fn current_set(&self) -> &std::collections::HashSet<T> {
        &self.current_set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_set_tracker() {
        let mut tracker = SetTracker::new();
        
        // Test insertions
        assert!(tracker.insert("a"));
        assert!(!tracker.insert("a")); // Duplicate
        assert!(tracker.insert("b"));
        
        assert_eq!(tracker.addition_history(), &["a", "b"]);
        assert!(tracker.removal_history().is_empty());
        assert_eq!(tracker.current_set().len(), 2);
        
        // Test removals
        assert!(tracker.remove(&"a"));
        assert!(!tracker.remove(&"a")); // Already removed
        
        assert_eq!(tracker.removal_history(), &["a"]);
        assert_eq!(tracker.current_set().len(), 1);
        assert!(tracker.contains(&"b"));
        assert!(!tracker.contains(&"a"));
    }

    #[test]
    fn test_set_tracker_empty() {
        let tracker: SetTracker<i32> = SetTracker::new();
        
        assert!(tracker.current_set().is_empty());
        assert!(tracker.addition_history().is_empty());
        assert!(tracker.removal_history().is_empty());
        assert!(!tracker.contains(&42));
    }

    #[test]
    fn test_set_tracker_multiple_operations() {
        let mut tracker = SetTracker::new();
        
        // Add several items
        assert!(tracker.insert(1));
        assert!(tracker.insert(2));
        assert!(tracker.insert(3));
        
        assert_eq!(tracker.current_set().len(), 3);
        assert_eq!(tracker.addition_history().len(), 3);
        
        // Remove some items
        assert!(tracker.remove(&2));
        assert!(tracker.remove(&3));
        
        assert_eq!(tracker.current_set().len(), 1);
        assert_eq!(tracker.removal_history().len(), 2);
        assert!(tracker.contains(&1));
        assert!(!tracker.contains(&2));
        assert!(!tracker.contains(&3));
    }

    #[test]
    fn test_set_tracker_readd_removed_item() {
        let mut tracker = SetTracker::new();
        
        // Add, remove, then add again
        assert!(tracker.insert("test"));
        assert!(tracker.remove(&"test"));
        assert!(tracker.insert("test")); // Should succeed again
        
        // History should reflect all operations
        assert_eq!(tracker.addition_history(), &["test", "test"]);
        assert_eq!(tracker.removal_history(), &["test"]);
        assert!(tracker.contains(&"test"));
        assert_eq!(tracker.current_set().len(), 1);
    }

    #[test]
    fn test_set_tracker_duplicate_operations() {
        let mut tracker = SetTracker::new();
        
        // Multiple attempts to insert same item
        assert!(tracker.insert("duplicate"));
        assert!(!tracker.insert("duplicate"));
        assert!(!tracker.insert("duplicate"));
        
        // Only one addition should be recorded
        assert_eq!(tracker.addition_history(), &["duplicate"]);
        assert_eq!(tracker.current_set().len(), 1);
        
        // Multiple attempts to remove same item
        assert!(tracker.remove(&"duplicate"));
        assert!(!tracker.remove(&"duplicate"));
        assert!(!tracker.remove(&"duplicate"));
        
        // Only one removal should be recorded
        assert_eq!(tracker.removal_history(), &["duplicate"]);
        assert!(tracker.current_set().is_empty());
    }

    #[test]
    fn test_set_tracker_strings() {
        let mut tracker = SetTracker::new();
        
        let words = vec!["hello", "world", "rust", "programming"];
        
        // Add all words
        for word in &words {
            assert!(tracker.insert(word.to_string()));
        }
        
        assert_eq!(tracker.current_set().len(), 4);
        
        // Remove every other word
        assert!(tracker.remove(&"hello".to_string()));
        assert!(tracker.remove(&"rust".to_string()));
        
        assert_eq!(tracker.current_set().len(), 2);
        assert!(tracker.contains(&"world".to_string()));
        assert!(tracker.contains(&"programming".to_string()));
        assert!(!tracker.contains(&"hello".to_string()));
        assert!(!tracker.contains(&"rust".to_string()));
        
        assert_eq!(tracker.removal_history().len(), 2);
    }

    #[test]
    fn test_set_tracker_numbers() {
        let mut tracker = SetTracker::new();
        
        // Add numbers 1-10
        for i in 1..=10 {
            assert!(tracker.insert(i));
        }
        
        assert_eq!(tracker.current_set().len(), 10);
        
        // Remove even numbers
        for i in (2..=10).step_by(2) {
            assert!(tracker.remove(&i));
        }
        
        assert_eq!(tracker.current_set().len(), 5);
        assert_eq!(tracker.removal_history().len(), 5);
        
        // Check that odd numbers remain
        for i in (1..=9).step_by(2) {
            assert!(tracker.contains(&i));
        }
        
        // Check that even numbers are gone
        for i in (2..=10).step_by(2) {
            assert!(!tracker.contains(&i));
        }
    }

    #[test]
    fn test_set_tracker_history_order() {
        let mut tracker = SetTracker::new();
        
        // Perform operations in specific order
        tracker.insert("first");
        tracker.insert("second");
        tracker.insert("third");
        tracker.remove(&"second");
        tracker.insert("fourth");
        tracker.remove(&"first");
        tracker.remove(&"third");
        
        // Check that history maintains order
        assert_eq!(tracker.addition_history(), &["first", "second", "third", "fourth"]);
        assert_eq!(tracker.removal_history(), &["second", "first", "third"]);
        
        // Only fourth should remain
        assert_eq!(tracker.current_set().len(), 1);
        assert!(tracker.contains(&"fourth"));
    }

    #[test]
    fn test_set_tracker_large_dataset() {
        let mut tracker = SetTracker::new();
        
        // Add 100 items
        for i in 0..100 {
            assert!(tracker.insert(i));
        }
        
        assert_eq!(tracker.current_set().len(), 100);
        assert_eq!(tracker.addition_history().len(), 100);
        
        // Remove first 50 items
        for i in 0..50 {
            assert!(tracker.remove(&i));
        }
        
        assert_eq!(tracker.current_set().len(), 50);
        assert_eq!(tracker.removal_history().len(), 50);
        
        // Verify remaining items are 50-99
        for i in 50..100 {
            assert!(tracker.contains(&i));
        }
        
        for i in 0..50 {
            assert!(!tracker.contains(&i));
        }
    }

    #[test]
    fn test_set_tracker_complex_type() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct TestItem {
            id: u32,
            name: String,
        }
        
        let mut tracker = SetTracker::new();
        
        let item1 = TestItem { id: 1, name: "Item One".to_string() };
        let item2 = TestItem { id: 2, name: "Item Two".to_string() };
        
        assert!(tracker.insert(item1.clone()));
        assert!(tracker.insert(item2.clone()));
        assert_eq!(tracker.current_set().len(), 2);
        
        assert!(tracker.remove(&item1));
        assert_eq!(tracker.current_set().len(), 1);
        assert!(!tracker.contains(&item1));
        assert!(tracker.contains(&item2));
    }
}