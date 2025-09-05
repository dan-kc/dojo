// Mock Service Practice
//
// Learning objectives:
// - Using interior mutability for test mocks
// - RefCell for tracking method calls
// - Interior mutability patterns in testing
//
// Run with: cargo test mock_service

use std::cell::RefCell;

/// Create a mock object that tracks method calls using interior mutability.
pub struct MockService {
    call_log: RefCell<Vec<String>>,
}

impl MockService {
    /// Create a new mock service
    pub fn new() -> Self {
        todo!("Initialize with empty call log")
    }
    
    /// Simulate a method call, logging it internally
    pub fn call_method(&self, method_name: &str, args: &str) -> String {
        todo!("Log the method call and return a mock response")
    }
    
    /// Get the number of method calls made
    pub fn call_count(&self) -> usize {
        todo!("Return length of call log")
    }
    
    /// Get all method calls made (for verification in tests)
    pub fn get_call_log(&self) -> Vec<String> {
        todo!("Clone and return the call log")
    }
    
    /// Clear the call log
    pub fn reset(&self) {
        todo!("Clear the call log")
    }
    
    /// Check if a specific method was called
    pub fn was_called(&self, method_name: &str) -> bool {
        todo!("Check if any call log entry contains the method name")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_service_basic() {
        let mock = MockService::new();
        assert_eq!(mock.call_count(), 0);
        
        let response1 = mock.call_method("login", "user=alice");
        let response2 = mock.call_method("get_data", "id=123");
        
        assert_eq!(mock.call_count(), 2);
        assert!(mock.was_called("login"));
        assert!(mock.was_called("get_data"));
        assert!(!mock.was_called("delete"));
    }

    #[test]
    fn test_mock_service_call_log() {
        let mock = MockService::new();
        
        mock.call_method("method1", "arg1");
        mock.call_method("method2", "arg2");
        
        let log = mock.get_call_log();
        assert_eq!(log.len(), 2);
        assert!(log[0].contains("method1"));
        assert!(log[1].contains("method2"));
    }

    #[test]
    fn test_mock_service_reset() {
        let mock = MockService::new();
        
        mock.call_method("test", "args");
        assert_eq!(mock.call_count(), 1);
        
        mock.reset();
        assert_eq!(mock.call_count(), 0);
        assert!(!mock.was_called("test"));
    }
}