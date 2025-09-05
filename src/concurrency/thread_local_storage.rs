// Thread-Local Storage Practice
//
// Learning Objectives:
// - Work with thread_local! macro
// - Understand non-Send/non-Sync patterns with thread-local data
// - Practice thread-local state management
//
// Run with: cargo test --bin thread_local_storage

use std::cell::Cell;
use std::thread;

/// Thread-local storage example that demonstrates non-Send/non-Sync patterns
thread_local! {
    static THREAD_LOCAL_COUNTER: Cell<i32> = Cell::new(0);
}

pub fn increment_thread_local() {
    todo!("Implement increment_thread_local")
}

pub fn get_thread_local() -> i32 {
    todo!("Implement get_thread_local")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_local_storage() {
        let handles: Vec<_> = (0..3).map(|thread_id| {
            thread::spawn(move || {
                // Each thread has its own counter
                for _ in 0..10 {
                    increment_thread_local();
                }
                (thread_id, get_thread_local())
            })
        }).collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        
        // Each thread should have count 10
        for (thread_id, count) in results {
            assert_eq!(count, 10, "Thread {} should have count 10", thread_id);
        }

        // Main thread should still have count 0
        assert_eq!(get_thread_local(), 0);
    }
}