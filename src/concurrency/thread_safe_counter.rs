// Thread-Safe Counter Practice
//
// Learning Objectives:
// - Implement Send + Sync traits for thread-safe types
// - Use Arc<Mutex<T>> for shared mutable state
// - Practice thread-safe operations
//
// Run with: cargo test --bin thread_safe_counter

use std::sync::{Arc, Mutex};
use std::thread;

/// A thread-safe counter that implements Send + Sync
pub struct ThreadSafeCounter {
    // Define your fields here
}

impl ThreadSafeCounter {
    pub fn new(initial: i32) -> Self {
        todo!("Implement new")
    }

    pub fn increment(&self) {
        todo!("Implement increment")
    }

    pub fn get(&self) -> i32 {
        todo!("Implement get")
    }

    pub fn add(&self, value: i32) {
        todo!("Implement add")
    }
}

/// Test helper to verify Send + Sync requirements at compile time
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
fn assert_send_sync<T: Send + Sync>() {}

/// Utility functions to work with Send/Sync constraints
pub fn share_across_threads<T: Send + Sync + 'static>(value: T) -> Arc<T> {
    Arc::new(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_bounds() {
        // Verify our types have the expected Send/Sync properties
        assert_send::<ThreadSafeCounter>();
        assert_sync::<ThreadSafeCounter>();
        assert_send_sync::<ThreadSafeCounter>();
    }

    #[test]
    fn test_thread_safe_counter() {
        let counter = Arc::new(ThreadSafeCounter::new(0));
        let mut handles = Vec::new();

        // Spawn multiple threads that increment the counter
        for _ in 0..10 {
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter_clone.increment();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 1000);
    }

    #[test]
    fn test_share_across_threads() {
        let shared = share_across_threads(ThreadSafeCounter::new(0));
        let shared_clone = shared.clone();

        let handle = thread::spawn(move || {
            shared_clone.add(50);
        });

        shared.add(25);
        handle.join().unwrap();

        assert_eq!(shared.get(), 75);
    }
}