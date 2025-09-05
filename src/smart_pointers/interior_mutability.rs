// Interior Mutability Practice
//
// Learning objectives:
// - Understanding RefCell<T> and runtime borrow checking
// - Using Cell<T> for Copy types
// - Combining Rc<T> with RefCell<T>
// - Understanding the trade-offs of interior mutability
//
// cargo test --lib smart_pointers::interior_mutability

use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Implement a simple counter using RefCell for interior mutability.
/// The counter should be shareable and mutable even through immutable references.
pub struct Counter {
    // TODO: Add field: count: RefCell<usize>
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

/// Create a shared mutable list using Rc<RefCell<Vec<T>>>.
/// Multiple owners can modify the same list.
#[derive(Debug)]
pub struct SharedList<T> {
    // TODO: Add field: data: Rc<RefCell<Vec<T>>>
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> SharedList<T> {
    /// Create a new empty shared list
    pub fn new() -> Self {
        todo!("Create SharedList with Rc<RefCell<Vec<T>>>")
    }
    
    /// Create a new shared list from existing data
    pub fn from_vec(vec: Vec<T>) -> Self {
        todo!("Wrap provided vector in Rc<RefCell<...>>")
    }
    
    /// Clone the shared list (creates new handle to same data)
    pub fn clone_handle(&self) -> Self {
        todo!("Clone the Rc, not the data")
    }
    
    /// Push an item to the list
    pub fn push(&self, item: T) {
        todo!("Borrow mutably and push item")
    }
    
    /// Pop an item from the list
    pub fn pop(&self) -> Option<T> {
        todo!("Borrow mutably and pop item")
    }
    
    /// Get the length of the list
    pub fn len(&self) -> usize {
        todo!("Borrow immutably and return length")
    }
    
    /// Get a copy of the item at index (if T: Clone)
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        todo!("Borrow immutably and clone item at index if it exists")
    }
    
    /// Apply a function to each element in the list
    pub fn for_each<F>(&self, mut func: F)
    where
        F: FnMut(&T),
    {
        todo!("Borrow immutably and apply function to each element")
    }
    
    /// Transform each element in the list using a function
    pub fn map_in_place<F>(&self, func: F)
    where
        F: Fn(&T) -> T,
        T: Clone,
    {
        todo!("Borrow mutably, apply function to each element, update in place")
    }
}

/// Implement a cache with interior mutability that can be used through immutable references.
pub struct MutableCache<K, V> {
    // TODO: Add field: cache: RefCell<std::collections::HashMap<K, V>>
    cache: RefCell<std::collections::HashMap<K, V>>,
}

impl<K, V> MutableCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new empty cache
    pub fn new() -> Self {
        todo!("Initialize with empty HashMap in RefCell")
    }
    
    /// Get or compute a value for the given key
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        todo!("Check if key exists, if not compute and insert, then return value")
    }
    
    /// Insert a value for the given key
    pub fn insert(&self, key: K, value: V) {
        todo!("Borrow mutably and insert key-value pair")
    }
    
    /// Check if the cache contains a key
    pub fn contains_key(&self, key: &K) -> bool {
        todo!("Borrow immutably and check if key exists")
    }
    
    /// Clear all entries from the cache
    pub fn clear(&self) {
        todo!("Borrow mutably and clear HashMap")
    }
    
    /// Get the number of cached entries
    pub fn len(&self) -> usize {
        todo!("Borrow immutably and return HashMap length")
    }
}

/// Demonstrate Cell<T> for Copy types - a simpler alternative to RefCell<T>
pub struct CellCounter {
    // TODO: Add field: count: Cell<i32>
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

/// Create a mock object that tracks method calls using interior mutability.
pub struct MockService {
    // TODO: Add field: call_log: RefCell<Vec<String>>
    call_log: RefCell<Vec<String>>,
}

impl MockService {
    /// Create a new mock service
    pub fn new() -> Self {
        todo!("Initialize with empty call log")
    }
    
    /// Simulate a method call, logging it internally
    pub fn call_method(&self, method_name: &str, args: &str) -> String {
        todo!("Log the method call and return a mock response")
    }
    
    /// Get the number of method calls made
    pub fn call_count(&self) -> usize {
        todo!("Return length of call log")
    }
    
    /// Get all method calls made (for verification in tests)
    pub fn get_call_log(&self) -> Vec<String> {
        todo!("Clone and return the call log")
    }
    
    /// Clear the call log
    pub fn reset(&self) {
        todo!("Clear the call log")
    }
    
    /// Check if a specific method was called
    pub fn was_called(&self, method_name: &str) -> bool {
        todo!("Check if any call log entry contains the method name")
    }
}

/// Demonstrate potential runtime panics with RefCell borrow checking
pub fn demonstrate_borrow_checking() -> Result<String, &'static str> {
    todo!("Create RefCell, try to create conflicting borrows, handle the panic gracefully")
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
    fn test_counter_immutable_reference() {
        let counter = Counter::new();
        
        // Even through an immutable reference, we can modify the counter
        fn increment_through_immutable_ref(c: &Counter) {
            c.increment();
            c.add(10);
        }
        
        increment_through_immutable_ref(&counter);
        assert_eq!(counter.get(), 11);
    }

    #[test]
    fn test_shared_list_basic() {
        let list = SharedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(list.len(), 3);
        assert_eq!(list.get(1), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_shared_list_multiple_handles() {
        let list1 = SharedList::from_vec(vec!["a", "b"]);
        let list2 = list1.clone_handle();
        
        list1.push("c");
        list2.push("d");
        
        // Both handles see the same data
        assert_eq!(list1.len(), 4);
        assert_eq!(list2.len(), 4);
        
        assert_eq!(list1.get(3), Some("d"));
        assert_eq!(list2.get(2), Some("c"));
    }

    #[test]
    fn test_shared_list_for_each() {
        let list = SharedList::from_vec(vec![1, 2, 3, 4]);
        let mut sum = 0;
        
        list.for_each(|&x| sum += x);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_shared_list_map_in_place() {
        let list = SharedList::from_vec(vec![1, 2, 3]);
        list.map_in_place(|&x| x * 2);
        
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), Some(4));
        assert_eq!(list.get(2), Some(6));
    }

    #[test]
    fn test_mutable_cache_basic() {
        let cache: MutableCache<String, i32> = MutableCache::new();
        
        cache.insert("key1".to_string(), 100);
        assert!(cache.contains_key(&"key1".to_string()));
        assert_eq!(cache.len(), 1);
        
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(!cache.contains_key(&"key1".to_string()));
    }

    #[test]
    fn test_mutable_cache_get_or_insert_with() {
        let cache: MutableCache<i32, String> = MutableCache::new();
        
        let result1 = cache.get_or_insert_with(1, || "computed_value".to_string());
        assert_eq!(result1, "computed_value");
        assert_eq!(cache.len(), 1);
        
        // Second call should return cached value, not compute again
        let result2 = cache.get_or_insert_with(1, || "should_not_compute".to_string());
        assert_eq!(result2, "computed_value");
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_cell_counter() {
        let counter = CellCounter::new(5);
        assert_eq!(counter.get(), 5);
        
        counter.set(10);
        assert_eq!(counter.get(), 10);
        
        let old_value = counter.increment();
        assert_eq!(old_value, 10);
        assert_eq!(counter.get(), 11);
        
        let new_value = counter.add(5);
        assert_eq!(new_value, 16);
        assert_eq!(counter.get(), 16);
    }

    #[test]
    fn test_mock_service() {
        let mock = MockService::new();
        
        assert_eq!(mock.call_count(), 0);
        assert!(!mock.was_called("test_method"));
        
        let response1 = mock.call_method("login", "user=alice");
        let response2 = mock.call_method("get_data", "id=123");
        
        assert_eq!(mock.call_count(), 2);
        assert!(mock.was_called("login"));
        assert!(mock.was_called("get_data"));
        assert!(!mock.was_called("logout"));
        
        let log = mock.get_call_log();
        assert_eq!(log.len(), 2);
        assert!(log[0].contains("login"));
        assert!(log[1].contains("get_data"));
        
        mock.reset();
        assert_eq!(mock.call_count(), 0);
    }

    #[test]
    fn test_demonstrate_borrow_checking() {
        let result = demonstrate_borrow_checking();
        // Should either succeed or return an error, not panic
        match result {
            Ok(_) => {
                // Success case
            }
            Err(msg) => {
                assert!(msg.contains("borrow"));
            }
        }
    }
}