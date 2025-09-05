// Hierarchical Locking Practice
//
// Learning Objectives:
// - Implement hierarchical locking to prevent deadlocks
// - Understand lock ordering by resource levels
// - Practice deadlock detection mechanisms
//
// Run with: cargo test --bin hierarchical_locking

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Implement a hierarchical locking system where resources have levels.
/// Higher-level resources must be acquired before lower-level ones.
pub struct HierarchicalLockManager {
    resources: Vec<Arc<Mutex<(usize, String)>>>, // (level, data)
}

impl HierarchicalLockManager {
    pub fn new(resources: Vec<(usize, String)>) -> Self {
        todo!("Implement new")
    }

    /// Acquire resources in hierarchical order (highest level first).
    pub fn acquire_hierarchical(&self, resource_ids: Vec<usize>) -> Option<Vec<String>> {
        todo!("Implement hierarchical acquisition")
    }
}

/// A utility function to detect potential deadlocks by monitoring lock wait times.
pub fn deadlock_detector<F>(operation: F, timeout: Duration) -> Result<(), &'static str>
where
    F: FnOnce() + Send + 'static,
{
    todo!("Implement deadlock detector")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_locking() {
        let resources = vec![
            (3, "High Priority".to_string()),
            (1, "Low Priority".to_string()),
            (2, "Medium Priority".to_string()),
        ];
        let manager = HierarchicalLockManager::new(resources);
        
        // Should acquire in hierarchical order
        let acquired = manager.acquire_hierarchical(vec![1, 0, 2]);
        assert!(acquired.is_some());
        
        let data = acquired.unwrap();
        // Should be ordered by hierarchy level (descending)
        assert_eq!(data[0], "High Priority");
        assert_eq!(data[1], "Medium Priority"); 
        assert_eq!(data[2], "Low Priority");
    }

    #[test]
    fn test_deadlock_detector() {
        // Test normal operation
        let result = deadlock_detector(|| {
            thread::sleep(Duration::from_millis(50));
        }, Duration::from_millis(100));
        assert!(result.is_ok());
        
        // Test timeout detection
        let result = deadlock_detector(|| {
            thread::sleep(Duration::from_millis(200));
        }, Duration::from_millis(100));
        assert!(result.is_err());
    }
}