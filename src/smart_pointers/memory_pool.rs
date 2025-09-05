// Memory Pool Implementation Practice
//
// Learning objectives:
// - Implementing custom memory management patterns
// - Using Box<T> for heap allocation and ownership
// - Combining Rc<RefCell<T>> for shared mutable access
// - Using Weak<T> references for tracking without ownership
// - Understanding object lifecycle management
// - Implementing resource reuse patterns
//
// Run with: cargo test memory_pool

use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Create a memory pool using smart pointers for efficient allocation/deallocation.
pub struct MemoryPool<T> {
    available: RefCell<Vec<Box<T>>>,
    allocated: RefCell<Vec<Weak<RefCell<T>>>>,
}

impl<T> MemoryPool<T>
where
    T: Default,
{
    /// Create a new memory pool with initial capacity
    pub fn new(initial_capacity: usize) -> Self {
        todo!("Pre-allocate boxes and store in available list")
    }
    
    /// Allocate an object from the pool
    pub fn allocate(&self) -> Rc<RefCell<T>> {
        todo!("Reuse from pool or create new, track in allocated list")
    }
    
    /// Get statistics about the pool
    pub fn stats(&self) -> (usize, usize) {
        todo!("Return (available_count, allocated_count)")
    }
    
    /// Perform garbage collection - remove deallocated objects from tracking
    pub fn collect(&self) {
        todo!("Remove weak references that can no longer be upgraded")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool_basic() {
        let pool = MemoryPool::<i32>::new(2);
        let (available, allocated) = pool.stats();
        assert_eq!(available, 2);
        assert_eq!(allocated, 0);
        
        let obj1 = pool.allocate();
        let obj2 = pool.allocate();
        
        *obj1.borrow_mut() = 42;
        *obj2.borrow_mut() = 24;
        
        let (available, allocated) = pool.stats();
        assert!(allocated >= 2); // May have more due to implementation details
        
        drop(obj1);
        pool.collect();
        
        let (available, allocated) = pool.stats();
        assert!(allocated >= 1);
    }

    #[test]
    fn test_memory_pool_reuse() {
        let pool = MemoryPool::<String>::new(1);
        
        // Allocate and modify an object
        let obj1 = pool.allocate();
        *obj1.borrow_mut() = "first".to_string();
        
        let (available_before, allocated_before) = pool.stats();
        assert_eq!(available_before, 0); // Should be depleted
        assert_eq!(allocated_before, 1);
        
        drop(obj1); // Release the object
        pool.collect(); // Clean up
        
        // Allocate again - should reuse the previous allocation
        let obj2 = pool.allocate();
        let (available_after, allocated_after) = pool.stats();
        
        // The exact behavior depends on implementation, but we should see reuse
        assert_eq!(allocated_after, 1);
    }

    #[test]
    fn test_memory_pool_expansion() {
        let pool = MemoryPool::<i32>::new(1);
        
        let obj1 = pool.allocate();
        let obj2 = pool.allocate(); // Should expand beyond initial capacity
        let obj3 = pool.allocate();
        
        *obj1.borrow_mut() = 1;
        *obj2.borrow_mut() = 2;
        *obj3.borrow_mut() = 3;
        
        let (available, allocated) = pool.stats();
        assert_eq!(allocated, 3);
        assert_eq!(available, 0); // All should be in use
        
        assert_eq!(*obj1.borrow(), 1);
        assert_eq!(*obj2.borrow(), 2);
        assert_eq!(*obj3.borrow(), 3);
    }

    #[test]
    fn test_memory_pool_collect_multiple() {
        let pool = MemoryPool::<f64>::new(0); // Start with empty pool
        
        let obj1 = pool.allocate();
        let obj2 = pool.allocate();
        let obj3 = pool.allocate();
        
        *obj1.borrow_mut() = 1.1;
        *obj2.borrow_mut() = 2.2;
        *obj3.borrow_mut() = 3.3;
        
        let (_, allocated_before) = pool.stats();
        assert_eq!(allocated_before, 3);
        
        // Drop some objects
        drop(obj1);
        drop(obj3);
        
        pool.collect();
        
        let (_, allocated_after) = pool.stats();
        assert_eq!(allocated_after, 1); // Only obj2 should remain
        
        // obj2 should still be accessible
        assert_eq!(*obj2.borrow(), 2.2);
    }

    #[test]
    fn test_memory_pool_zero_initial_capacity() {
        let pool = MemoryPool::<u32>::new(0);
        let (available, allocated) = pool.stats();
        assert_eq!(available, 0);
        assert_eq!(allocated, 0);
        
        // Should still be able to allocate
        let obj = pool.allocate();
        *obj.borrow_mut() = 100;
        
        let (available, allocated) = pool.stats();
        assert_eq!(available, 0);
        assert_eq!(allocated, 1);
        
        assert_eq!(*obj.borrow(), 100);
    }

    #[test]
    fn test_memory_pool_large_capacity() {
        let pool = MemoryPool::<bool>::new(100);
        let (available, allocated) = pool.stats();
        assert_eq!(available, 100);
        assert_eq!(allocated, 0);
        
        // Allocate a few objects
        let objects: Vec<_> = (0..5).map(|i| {
            let obj = pool.allocate();
            *obj.borrow_mut() = i % 2 == 0;
            obj
        }).collect();
        
        let (available, allocated) = pool.stats();
        assert_eq!(available, 95); // Should have 95 left
        assert_eq!(allocated, 5);
        
        // Verify values
        for (i, obj) in objects.iter().enumerate() {
            assert_eq!(*obj.borrow(), i % 2 == 0);
        }
    }

    #[test] 
    fn test_memory_pool_collect_no_effect_on_live_objects() {
        let pool = MemoryPool::<i32>::new(1);
        
        let obj1 = pool.allocate();
        let obj2 = pool.allocate();
        
        *obj1.borrow_mut() = 42;
        *obj2.borrow_mut() = 84;
        
        // Collect should not affect live objects
        pool.collect();
        
        let (_, allocated) = pool.stats();
        assert_eq!(allocated, 2);
        
        assert_eq!(*obj1.borrow(), 42);
        assert_eq!(*obj2.borrow(), 84);
    }

    // Test with custom type
    #[derive(Debug, PartialEq)]
    struct TestStruct {
        id: usize,
        data: String,
    }

    impl Default for TestStruct {
        fn default() -> Self {
            Self {
                id: 0,
                data: String::new(),
            }
        }
    }

    #[test]
    fn test_memory_pool_custom_type() {
        let pool = MemoryPool::<TestStruct>::new(2);
        
        let obj1 = pool.allocate();
        let obj2 = pool.allocate();
        
        obj1.borrow_mut().id = 1;
        obj1.borrow_mut().data = "first".to_string();
        
        obj2.borrow_mut().id = 2;
        obj2.borrow_mut().data = "second".to_string();
        
        assert_eq!(obj1.borrow().id, 1);
        assert_eq!(obj1.borrow().data, "first");
        assert_eq!(obj2.borrow().id, 2);
        assert_eq!(obj2.borrow().data, "second");
        
        let (available, allocated) = pool.stats();
        assert_eq!(available, 0);
        assert_eq!(allocated, 2);
    }
}