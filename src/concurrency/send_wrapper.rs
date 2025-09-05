// Send Wrapper Practice
//
// Learning Objectives:
// - Create wrappers around non-Send types to make them usable across threads
// - Use interior mutability for thread safety
// - Understand when and how to implement Send manually
//
// Run with: cargo test --bin send_wrapper

use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

/// A wrapper around a non-Send type that makes it usable across threads
/// by using interior mutability and ensuring thread safety
pub struct SendWrapper<T> {
    value: Mutex<T>,
}

impl<T> SendWrapper<T> {
    pub fn new(value: T) -> Self {
        todo!("Implement new")
    }

    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        todo!("Implement with")
    }

    pub fn with_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        todo!("Implement with_mut")
    }
}

// Implement Send for SendWrapper when T is Send
// This is typically done automatically, but we're being explicit for learning

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_wrapper() {
        let wrapper = Arc::new(SendWrapper::new(String::from("Hello, World!")));
        let wrapper_clone = wrapper.clone();

        let handle = thread::spawn(move || {
            wrapper_clone.with(|s| {
                assert_eq!(s, "Hello, World!");
                s.len()
            })
        });

        let length = handle.join().unwrap();
        assert_eq!(length, 13);

        // Test mutable access
        wrapper.with_mut(|s| {
            s.push_str(" Modified");
        });

        wrapper.with(|s| {
            assert_eq!(s, "Hello, World! Modified");
        });
    }
}