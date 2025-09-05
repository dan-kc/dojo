// Send but Not Sync Practice
//
// Learning Objectives:
// - Understand types that are Send but not Sync
// - Work with RefCell for interior mutability
// - Learn the difference between Send and Sync traits
//
// Run with: cargo test --bin send_not_sync

use std::cell::RefCell;
use std::thread;

/// A demonstration of a type that is Send but not Sync
pub struct SendNotSync {
    data: RefCell<i32>,
}

impl SendNotSync {
    pub fn new(value: i32) -> Self {
        todo!("Implement new")
    }

    pub fn get(&self) -> i32 {
        todo!("Implement get")
    }

    pub fn set(&self, value: i32) {
        todo!("Implement set")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_send_not_sync() {
        let send_not_sync = SendNotSync::new(100);
        
        // Can send to another thread
        let handle = thread::spawn(move || {
            send_not_sync.set(200);
            send_not_sync.get()
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 200);
        
        // But cannot share references across threads (would fail to compile)
        // let shared_ref = &send_not_sync;  // This would make the test fail to compile
    }

    #[test]
    fn test_refcell_behavior() {
        // RefCell is Send but not Sync due to interior mutability
        // let cell = Cell::new(5);  // Cell is Send but not Sync
        // let cell_ref = &cell;
        // thread::spawn(move || cell_ref.set(10)); // Would fail to compile
    }
}