// Reference Counting Practice
//
// Learning objectives:
// - Understanding Rc<T> for shared ownership
// - Working with Rc::clone() vs regular cloning
// - Using Weak<T> to break reference cycles
// - Arc<T> for thread-safe shared ownership
//
// cargo test --lib smart_pointers::reference_counting

use std::rc::{Rc, Weak};
use std::sync::Arc;

/// Implement a graph node that can have multiple parents using Rc<T>.
/// This demonstrates shared ownership patterns.
#[derive(Debug)]
pub struct GraphNode<T> {
    todo!("Add fields: value: T, children: Vec<Rc<GraphNode<T>>>, parent: Option<Weak<GraphNode<T>>>")
}

impl<T> GraphNode<T> {
    /// Create a new graph node with the given value
    pub fn new(value: T) -> Rc<Self> {
        todo!("Create new node wrapped in Rc")
    }
    
    /// Add a child node to this node
    pub fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
        todo!("Add child to parent's children list and set parent weak reference")
    }
    
    /// Get the value of this node
    pub fn value(&self) -> &T {
        todo!("Return reference to value")
    }
    
    /// Get the number of children
    pub fn child_count(&self) -> usize {
        todo!("Return length of children vector")
    }
    
    /// Get a child by index
    pub fn get_child(&self, index: usize) -> Option<Rc<Self>> {
        todo!("Return cloned Rc to child if index is valid")
    }
    
    /// Check if this node has a parent
    pub fn has_parent(&self) -> bool {
        todo!("Check if parent weak reference can be upgraded")
    }
    
    /// Get the parent node if it still exists
    pub fn get_parent(&self) -> Option<Rc<Self>> {
        todo!("Try to upgrade weak reference to parent")
    }
}

/// Create a function that demonstrates the difference between Rc::clone and T::clone
pub fn demonstrate_rc_cloning<T>(rc_value: Rc<T>) -> (Rc<T>, usize)
where
    T: Clone,
{
    todo!("Clone the Rc (cheap) and return it with the reference count")
}

/// Implement a shared cache using Rc<T> that can be shared between multiple owners.
#[derive(Debug)]
pub struct SharedCache<K, V> {
    todo!("Add field: data: std::collections::HashMap<K, Rc<V>>")
}

impl<K, V> SharedCache<K, V>
where
    K: std::hash::Hash + Eq,
{
    /// Create a new empty shared cache
    pub fn new() -> Self {
        todo!("Create cache with empty HashMap")
    }
    
    /// Insert a value into the cache, returning an Rc to it
    pub fn insert(&mut self, key: K, value: V) -> Rc<V> {
        todo!("Insert value wrapped in Rc and return clone of the Rc")
    }
    
    /// Get a value from the cache
    pub fn get(&self, key: &K) -> Option<Rc<V>> {
        todo!("Return cloned Rc if key exists")
    }
    
    /// Remove a value from the cache
    pub fn remove(&mut self, key: &K) -> Option<Rc<V>> {
        todo!("Remove and return cloned Rc if key existed")
    }
    
    /// Get the number of cached items
    pub fn len(&self) -> usize {
        todo!("Return HashMap length")
    }
}

/// Demonstrate Arc<T> for thread-safe shared ownership.
/// Create a function that spawns multiple threads that all read from the same data.
pub fn shared_data_across_threads(data: Vec<String>) -> Vec<usize> {
    todo!("Wrap data in Arc, spawn threads that each count strings containing 'rust', return results")
}

/// Create a cyclic reference example and show how to break it with Weak<T>.
pub struct CyclicNode<T> {
    todo!("Add fields: value: T, next: Option<Rc<CyclicNode<T>>>, prev: Option<Weak<CyclicNode<T>>>")
}

impl<T> CyclicNode<T> {
    /// Create a new cyclic node
    pub fn new(value: T) -> Rc<Self> {
        todo!("Create node wrapped in Rc")
    }
    
    /// Connect two nodes bidirectionally
    pub fn connect(first: &Rc<Self>, second: &Rc<Self>) {
        todo!("Set first.next = second and second.prev = first (using weak reference)")
    }
    
    /// Get the next node
    pub fn get_next(&self) -> Option<Rc<Self>> {
        todo!("Return cloned Rc to next node")
    }
    
    /// Get the previous node if it still exists
    pub fn get_prev(&self) -> Option<Rc<Self>> {
        todo!("Try to upgrade weak reference to previous node")
    }
    
    /// Get the value
    pub fn value(&self) -> &T {
        todo!("Return reference to value")
    }
}

/// Create a function that demonstrates Weak<T> preventing memory leaks.
/// Build a circular linked list and show it can be properly cleaned up.
pub fn create_circular_list<T>(values: Vec<T>) -> Option<Rc<CyclicNode<T>>> {
    todo!("Create circular linked list where each node points to next, last points to first")
}

/// Implement a function that counts strong and weak references to demonstrate
/// the difference between Rc::strong_count and Rc::weak_count.
pub fn reference_count_demo<T>(value: T) -> (usize, usize, usize) {
    todo!("Create Rc, clone it twice, create one weak reference, return (strong, weak, strong)")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_graph_node_creation() {
        let node = GraphNode::new(42);
        assert_eq!(*node.value(), 42);
        assert_eq!(node.child_count(), 0);
        assert!(!node.has_parent());
    }

    #[test]
    fn test_graph_node_parent_child() {
        let parent = GraphNode::new("parent");
        let child = GraphNode::new("child");
        
        GraphNode::add_child(&parent, child.clone());
        
        assert_eq!(parent.child_count(), 1);
        assert!(child.has_parent());
        
        let retrieved_child = parent.get_child(0).unwrap();
        assert_eq!(*retrieved_child.value(), "child");
        
        let retrieved_parent = child.get_parent().unwrap();
        assert_eq!(*retrieved_parent.value(), "parent");
    }

    #[test]
    fn test_graph_node_multiple_children() {
        let parent = GraphNode::new(0);
        let child1 = GraphNode::new(1);
        let child2 = GraphNode::new(2);
        let child3 = GraphNode::new(3);
        
        GraphNode::add_child(&parent, child1);
        GraphNode::add_child(&parent, child2);
        GraphNode::add_child(&parent, child3);
        
        assert_eq!(parent.child_count(), 3);
        assert_eq!(*parent.get_child(1).unwrap().value(), 2);
        assert!(parent.get_child(5).is_none());
    }

    #[test]
    fn test_demonstrate_rc_cloning() {
        let original = Rc::new(String::from("test"));
        let original_count = Rc::strong_count(&original);
        
        let (cloned, count) = demonstrate_rc_cloning(original.clone());
        
        assert_eq!(*cloned, "test");
        assert_eq!(count, original_count + 1); // Should be incremented
    }

    #[test]
    fn test_shared_cache_basic() {
        let mut cache = SharedCache::new();
        
        let value1 = cache.insert("key1", "value1");
        let value2 = cache.insert("key2", "value2");
        
        assert_eq!(cache.len(), 2);
        assert_eq!(*value1, "value1");
        assert_eq!(*value2, "value2");
        
        let retrieved = cache.get(&"key1").unwrap();
        assert_eq!(*retrieved, "value1");
        
        assert!(cache.get(&"nonexistent").is_none());
    }

    #[test]
    fn test_shared_cache_remove() {
        let mut cache = SharedCache::new();
        cache.insert(1, "one");
        cache.insert(2, "two");
        
        let removed = cache.remove(&1).unwrap();
        assert_eq!(*removed, "one");
        assert_eq!(cache.len(), 1);
        assert!(cache.get(&1).is_none());
        
        assert!(cache.remove(&999).is_none());
    }

    #[test]
    fn test_shared_data_across_threads() {
        let data = vec![
            "hello world".to_string(),
            "rust programming".to_string(),
            "not related".to_string(),
            "rust is awesome".to_string(),
            "java programming".to_string(),
        ];
        
        let results = shared_data_across_threads(data);
        
        // Each thread should count strings containing "rust"
        // There are 2 such strings, so each thread should return 2
        assert!(results.len() > 0);
        for &count in &results {
            assert_eq!(count, 2);
        }
    }

    #[test]
    fn test_cyclic_node_creation() {
        let node = CyclicNode::new(42);
        assert_eq!(*node.value(), 42);
        assert!(node.get_next().is_none());
        assert!(node.get_prev().is_none());
    }

    #[test]
    fn test_cyclic_node_connection() {
        let node1 = CyclicNode::new(1);
        let node2 = CyclicNode::new(2);
        
        CyclicNode::connect(&node1, &node2);
        
        assert_eq!(*node1.get_next().unwrap().value(), 2);
        assert_eq!(*node2.get_prev().unwrap().value(), 1);
    }

    #[test]
    fn test_create_circular_list() {
        let values = vec![1, 2, 3];
        let head = create_circular_list(values);
        
        assert!(head.is_some());
        let head = head.unwrap();
        
        // Follow the circle: 1 -> 2 -> 3 -> 1
        let second = head.get_next().unwrap();
        assert_eq!(*second.value(), 2);
        
        let third = second.get_next().unwrap();
        assert_eq!(*third.value(), 3);
        
        let back_to_first = third.get_next().unwrap();
        assert_eq!(*back_to_first.value(), 1);
    }

    #[test]
    fn test_create_circular_list_empty() {
        let values: Vec<i32> = vec![];
        let result = create_circular_list(values);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_circular_list_single() {
        let values = vec![42];
        let head = create_circular_list(values).unwrap();
        
        // Single node should point to itself
        let next = head.get_next().unwrap();
        assert_eq!(*next.value(), 42);
    }

    #[test]
    fn test_reference_count_demo() {
        let (strong1, weak1, strong2) = reference_count_demo(String::from("test"));
        
        // Should have created 2 strong references initially, then cloned twice more
        // Plus one weak reference
        assert_eq!(strong1, 3); // Original + 2 clones
        assert_eq!(weak1, 1);   // One weak reference
        assert_eq!(strong2, 3); // Same strong count
    }
}