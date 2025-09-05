// Send and Sync Traits Practice
//
// Learning Objectives:
// - Understand Send and Sync trait requirements
// - Implement custom types that work with threading
// - Handle non-Send/non-Sync types safely
// - Practice unsafe implementations when necessary
// - Work with thread-local storage
//
// Run with: cargo test --bin send_sync_traits

use std::sync::{Arc, Mutex};
use std::thread;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

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

/// A wrapper around a non-Send type that makes it usable across threads
/// by using interior mutability and ensuring thread safety
pub struct SendWrapper<T> {
    // Define your fields here
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

/// A struct that contains non-Send data but can still be used safely
/// across threads through careful encapsulation
pub struct NonSendContainer {
    // This contains Rc which is not Send, but we'll make the container Send
    data: Mutex<Option<Rc<String>>>,
}

impl NonSendContainer {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    pub fn set_data(&self, data: String) {
        todo!("Implement set_data")
    }

    pub fn get_data(&self) -> Option<String> {
        todo!("Implement get_data")
    }
}

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

/// A custom smart pointer that implements Send + Sync appropriately
pub struct CustomSharedPtr<T> {
    // Define your fields here
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

/// Test helper to verify Send + Sync requirements at compile time
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
fn assert_send_sync<T: Send + Sync>() {}

/// Utility functions to work with Send/Sync constraints
pub fn share_across_threads<T: Send + Sync + 'static>(value: T) -> Arc<T> {
    Arc::new(value)
}

pub fn send_to_thread<T: Send + 'static, F: FnOnce(T) -> T + Send + 'static>(
    value: T,
    processor: F,
) -> T {
    todo!("Implement send_to_thread")
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

        assert_send::<SendWrapper<String>>();
        // SendWrapper<T> should be Send when T is Send

        assert_send::<NonSendContainer>();
        assert_sync::<NonSendContainer>();

        assert_send::<SendNotSync>();
        // SendNotSync should be Send but not Sync due to RefCell
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

    #[test]
    fn test_non_send_container() {
        let container = Arc::new(NonSendContainer::new());
        let container_clone = container.clone();

        let handle = thread::spawn(move || {
            container_clone.set_data("Thread data".to_string());
            container_clone.get_data()
        });

        let result = handle.join().unwrap();
        assert_eq!(result, Some("Thread data".to_string()));
    }

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

    #[test]
    fn test_send_to_thread() {
        let result = send_to_thread(vec![1, 2, 3, 4, 5], |mut v| {
            v.iter_mut().for_each(|x| *x *= 2);
            v
        });

        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_compile_time_safety() {
        // These should compile (Send + Sync types)
        let _: Arc<ThreadSafeCounter> = Arc::new(ThreadSafeCounter::new(0));
        let _: Arc<NonSendContainer> = Arc::new(NonSendContainer::new());
        
        // These demonstrate the compiler preventing unsafe sharing
        // Uncommenting these would cause compilation errors:
        
        // let rc = Rc::new(5);  // Rc is not Send
        // thread::spawn(move || println!("{}", rc)); // Would fail to compile
        
        // let cell = Cell::new(5);  // Cell is Send but not Sync
        // let cell_ref = &cell;
        // thread::spawn(move || cell_ref.set(10)); // Would fail to compile
    }
}