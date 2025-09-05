// RefCell Counter Practice
//
// Learning objectives:
// - Understanding RefCell<T> and runtime borrow checking
// - Using interior mutability for shared mutable state
// - Safe mutation through immutable references
//
// Run with: cargo test refcell_counter

use std::cell::RefCell;

/// Implement a simple counter using RefCell for interior mutability.
/// The counter should be shareable and mutable even through immutable references.
pub struct Counter {
    count: RefCell<usize>,
}

impl Counter {
    /// Create a new counter starting at 0
    pub fn new() -> Self {
        todo!("Initialize counter with RefCell containing 0")
    }
    
    /// Increment the counter and return the new value
    pub fn increment(&self) -> usize {
        todo!("Borrow mutably, increment, and return new value")
    }
    
    /// Get the current count value
    pub fn get(&self) -> usize {
        todo!("Borrow immutably and return current value")
    }
    
    /// Add a specific amount to the counter
    pub fn add(&self, amount: usize) -> usize {
        todo!("Borrow mutably, add amount, return new value")
    }
    
    /// Reset the counter to 0
    pub fn reset(&self) {
        todo!("Borrow mutably and set to 0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_basic_operations() {
        let counter = Counter::new();
        assert_eq!(counter.get(), 0);
        
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.increment(), 2);
        assert_eq!(counter.get(), 2);
        
        assert_eq!(counter.add(5), 7);
        assert_eq!(counter.get(), 7);
        
        counter.reset();
        assert_eq!(counter.get(), 0);
    }

    #[test]
    fn test_counter_multiple_references() {
        let counter = Counter::new();
        let counter_ref1 = &counter;
        let counter_ref2 = &counter;
        
        // Both references should work on the same counter
        counter_ref1.increment();
        assert_eq!(counter_ref2.get(), 1);
        
        counter_ref2.add(3);
        assert_eq!(counter_ref1.get(), 4);
    }
}