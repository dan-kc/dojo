// cargo test --lib smart_pointers::cell_counter::

//! Mutate the counter through shared references. increment returns the old value; add returns the new value. Inputs and results fit i32. This is single-threaded interior mutability.

/// Demonstrate std::cell::Cell<T> for Copy types - a simpler alternative to RefCell<T>
pub struct CellCounter {
    count: std::cell::Cell<i32>,
}

impl CellCounter {
    /// Create a new cell counter
    pub fn new(initial: i32) -> Self {
        todo!()
    }

    /// Get the current value
    pub fn get(&self) -> i32 {
        todo!()
    }

    /// Set a new value
    pub fn set(&self, value: i32) {
        todo!()
    }

    /// Increment the counter and return old value
    pub fn increment(&self) -> i32 {
        todo!()
    }

    /// Add to the counter and return new value
    pub fn add(&self, amount: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

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
