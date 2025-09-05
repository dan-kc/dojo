// Timeout-Based Resource Management Practice
//
// Learning Objectives:
// - Implement timeout-based resource acquisition
// - Use try_lock patterns to avoid blocking
// - Handle resource contention gracefully
//
// Run with: cargo test --bin timeout_resource_manager

use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Implement a timeout-based resource acquisition system.
/// Resources have IDs and can be locked with timeouts.
pub struct TimeoutResourceManager {
    resources: Vec<Arc<Mutex<String>>>,
}

impl TimeoutResourceManager {
    pub fn new(resource_names: Vec<String>) -> Self {
        todo!("Implement new")
    }

    /// Try to acquire a resource with a timeout.
    /// Returns the resource content if successful, None if timeout.
    pub fn acquire_with_timeout(&self, resource_id: usize, timeout: Duration) -> Option<String> {
        todo!("Implement timeout-based acquisition")
    }

    /// Try to acquire multiple resources with timeout, using try_lock to avoid blocking.
    pub fn acquire_multiple_with_timeout(
        &self,
        resource_ids: Vec<usize>,
        timeout: Duration,
    ) -> Option<Vec<String>> {
        todo!("Implement multiple resource acquisition with timeout")
    }

    /// Demonstrate non-blocking resource acquisition.
    pub fn try_acquire_all(&self, resource_ids: Vec<usize>) -> Option<Vec<String>> {
        todo!("Implement non-blocking acquisition of all resources")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeout_resource_manager() {
        let resources = vec!["Resource1".to_string(), "Resource2".to_string(), "Resource3".to_string()];
        let manager = TimeoutResourceManager::new(resources);
        
        // Test successful acquisition
        let resource = manager.acquire_with_timeout(0, Duration::from_millis(100));
        assert_eq!(resource, Some("Resource1".to_string()));
        
        // Test multiple resource acquisition
        let resources = manager.acquire_multiple_with_timeout(vec![1, 2], Duration::from_millis(100));
        assert!(resources.is_some());
        assert_eq!(resources.unwrap().len(), 2);
        
        // Test try_acquire_all
        let all_resources = manager.try_acquire_all(vec![0, 1, 2]);
        assert!(all_resources.is_some());
    }
}