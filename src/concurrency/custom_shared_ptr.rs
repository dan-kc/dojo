// Custom Shared Pointer Practice
//
// Learning Objectives:
// - Implement custom smart pointers with Send + Sync
// - Work with Arc for reference counting
// - Understand smart pointer trait implementations
//
// Run with: cargo test --bin custom_shared_ptr

use std::sync::Arc;

/// A custom smart pointer that implements Send + Sync appropriately
pub struct CustomSharedPtr<T> {
    inner: Arc<T>,
}

impl<T> CustomSharedPtr<T> {
    pub fn new(value: T) -> Self {
        todo!("Implement new")
    }

    pub fn get(&self) -> Arc<T> {
        todo!("Implement get")
    }

    pub fn clone_ptr(&self) -> CustomSharedPtr<T> {
        todo!("Implement clone_ptr")
    }
}

// Implement Send and Sync for CustomSharedPtr
// We need to be explicit about when these traits are implemented

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_shared_ptr() {
        let ptr = CustomSharedPtr::new(42);
        let ptr_clone = ptr.clone_ptr();

        let original_arc = ptr.get();
        let cloned_arc = ptr_clone.get();

        assert_eq!(*original_arc, 42);
        assert_eq!(*cloned_arc, 42);
        
        // Should be the same underlying data
        assert!(Arc::ptr_eq(&original_arc, &cloned_arc));
    }
}