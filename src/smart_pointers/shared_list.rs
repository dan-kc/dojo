// Shared List Practice
//
// Learning objectives:
// - Combining Rc<T> with RefCell<T>
// - Multiple ownership with interior mutability
// - Understanding the trade-offs of shared mutable containers
//
// Run with: cargo test shared_list

use std::cell::RefCell;
use std::rc::Rc;

/// Create a shared mutable list using Rc<RefCell<Vec<T>>>.
/// Multiple owners can modify the same list.
#[derive(Debug)]
pub struct SharedList<T> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_list_basic_operations() {
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
        let list1 = SharedList::from_vec(vec![1, 2, 3]);
        let list2 = list1.clone_handle();
        
        // Both handles point to the same data
        list1.push(4);
        assert_eq!(list2.len(), 4);
        assert_eq!(list2.get(3), Some(4));
        
        list2.push(5);
        assert_eq!(list1.len(), 5);
        assert_eq!(list1.get(4), Some(5));
    }

    #[test]
    fn test_shared_list_transformations() {
        let list = SharedList::from_vec(vec![1, 2, 3, 4]);
        
        let mut sum = 0;
        list.for_each(|x| sum += x);
        assert_eq!(sum, 10);
        
        // Double all values
        list.map_in_place(|x| x * 2);
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), Some(4));
        assert_eq!(list.get(2), Some(6));
        assert_eq!(list.get(3), Some(8));
    }
}