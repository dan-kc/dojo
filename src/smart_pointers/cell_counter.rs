// Cell Counter Practice
//
// Learning objectives:
// - Using Cell<T> for Copy types
// - Understanding the difference between Cell<T> and RefCell<T>
// - Simple interior mutability patterns
//
// Run with: cargo test cell_counter

use std::cell::Cell;

/// Demonstrate Cell<T> for Copy types - a simpler alternative to RefCell<T>
pub struct CellCounter {
    count: Cell<i32>,
}

impl CellCounter {
    /// Create a new cell counter
    pub fn new(initial: i32) -> Self {
        todo!("Initialize with Cell containing initial value")
    }
    
    /// Get the current value
    pub fn get(&self) -> i32 {
        todo!("Use Cell::get() to retrieve value")
    }
    
    /// Set a new value
    pub fn set(&self, value: i32) {
        todo!("Use Cell::set() to update value")
    }
    
    /// Increment the counter and return old value
    pub fn increment(&self) -> i32 {
        todo!("Get current value, increment, set new value, return old value")
    }
    
    /// Add to the counter and return new value
    pub fn add(&self, amount: i32) -> i32 {
        todo!("Get current, add amount, set new value, return new value")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_counter_basic() {
        let counter = CellCounter::new(5);
        assert_eq!(counter.get(), 5);
        
        counter.set(10);
        assert_eq!(counter.get(), 10);
    }

    #[test]
    fn test_cell_counter_increment() {
        let counter = CellCounter::new(0);
        
        // increment returns old value
        assert_eq!(counter.increment(), 0);
        assert_eq!(counter.get(), 1);
        
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_cell_counter_add() {
        let counter = CellCounter::new(10);
        
        // add returns new value
        assert_eq!(counter.add(5), 15);
        assert_eq!(counter.get(), 15);
        
        assert_eq!(counter.add(-3), 12);
        assert_eq!(counter.get(), 12);
    }

    #[test]
    fn test_cell_counter_multiple_refs() {
        let counter = CellCounter::new(100);
        let ref1 = &counter;
        let ref2 = &counter;
        
        // Both references can mutate through Cell
        ref1.set(200);
        assert_eq!(ref2.get(), 200);
        
        ref2.add(50);
        assert_eq!(ref1.get(), 250);
    }
}