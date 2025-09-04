// Smart Pointer Combinations Practice
//
// Learning objectives:
// - Combining different smart pointer types effectively
// - Real-world patterns and trade-offs
// - When to use which combination
// - Performance and safety considerations
//
// cargo test --lib smart_pointers::smart_pointer_combinations

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

/// Implement a thread-safe shared cache using Arc<Mutex<T>>.
/// Multiple threads can safely read and write to the same cache.
pub struct ThreadSafeCache<K, V> {
    todo!("Add field: data: Arc<Mutex<HashMap<K, V>>>")
}

impl<K, V> ThreadSafeCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new thread-safe cache
    pub fn new() -> Self {
        todo!("Initialize with Arc<Mutex<HashMap>>")
    }
    
    /// Insert a key-value pair
    pub fn insert(&self, key: K, value: V) {
        todo!("Lock mutex and insert")
    }
    
    /// Get a value by key
    pub fn get(&self, key: &K) -> Option<V> {
        todo!("Lock mutex and get cloned value")
    }
    
    /// Get or compute a value
    pub fn get_or_insert_with<F>(&self, key: K, compute: F) -> V
    where
        F: FnOnce() -> V,
    {
        todo!("Lock once, check if exists, compute if needed, insert and return")
    }
    
    /// Clone the cache handle (shares the same underlying data)
    pub fn clone_handle(&self) -> Self {
        todo!("Clone the Arc")
    }
    
    /// Clear all entries
    pub fn clear(&self) {
        todo!("Lock mutex and clear HashMap")
    }
    
    /// Get the number of entries
    pub fn len(&self) -> usize {
        todo!("Lock mutex and return HashMap length")
    }
}

/// Create a complex tree structure with parent-child relationships using multiple smart pointers.
/// Parents hold strong references to children, children hold weak references to parents.
pub struct TreeNodeComplex<T> {
    todo!("Add fields: value: T, children: RefCell<Vec<Rc<TreeNodeComplex<T>>>>, parent: RefCell<Option<Weak<TreeNodeComplex<T>>>>")
}

impl<T> TreeNodeComplex<T> {
    /// Create a new tree node
    pub fn new(value: T) -> Rc<Self> {
        todo!("Create node wrapped in Rc")
    }
    
    /// Add a child to this node
    pub fn add_child(parent: &Rc<Self>, child: Rc<Self>) {
        todo!("Add child to parent's children list and set child's parent")
    }
    
    /// Remove a child by value (if T: PartialEq)
    pub fn remove_child(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        todo!("Find and remove child with matching value, clear its parent")
    }
    
    /// Get all children values
    pub fn get_children_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        todo!("Collect values from all children")
    }
    
    /// Find a node by value using depth-first search
    pub fn find_node(&self, target: &T) -> Option<Rc<Self>>
    where
        T: PartialEq,
    {
        todo!("Search this node and recursively search children")
    }
    
    /// Get the depth of this node (distance from root)
    pub fn get_depth(&self) -> usize {
        todo!("Follow parent links to count depth")
    }
    
    /// Get path from root to this node
    pub fn get_path_from_root(&self) -> Vec<T>
    where
        T: Clone,
    {
        todo!("Follow parent links to build path from root")
    }
}

/// Implement a publish-subscribe system using smart pointers.
/// Publishers can notify multiple subscribers, subscribers can be dropped independently.
pub struct Publisher<T> {
    todo!("Add field: subscribers: RefCell<Vec<Weak<dyn Subscriber<T>>>>")
}

pub trait Subscriber<T> {
    fn notify(&self, message: &T);
}

impl<T> Publisher<T>
where
    T: Clone,
{
    /// Create a new publisher
    pub fn new() -> Self {
        todo!("Initialize with empty subscribers list")
    }
    
    /// Subscribe to this publisher
    pub fn subscribe(&self, subscriber: Rc<dyn Subscriber<T>>) {
        todo!("Add weak reference to subscriber to the list")
    }
    
    /// Publish a message to all active subscribers
    pub fn publish(&self, message: T) {
        todo!("Notify all subscribers that can still be upgraded from weak references")
    }
    
    /// Clean up dropped subscribers
    pub fn cleanup_subscribers(&self) {
        todo!("Remove weak references that can no longer be upgraded")
    }
    
    /// Get the number of active subscribers
    pub fn active_subscriber_count(&self) -> usize {
        todo!("Count weak references that can still be upgraded")
    }
}

/// Example subscriber implementation
pub struct LoggingSubscriber {
    todo!("Add field: name: String, logs: RefCell<Vec<String>>")
}

impl LoggingSubscriber {
    pub fn new(name: String) -> Rc<Self> {
        todo!("Create subscriber wrapped in Rc")
    }
    
    pub fn get_logs(&self) -> Vec<String> {
        todo!("Return cloned logs")
    }
}

impl Subscriber<String> for LoggingSubscriber {
    fn notify(&self, message: &String) {
        todo!("Log the message with subscriber name")
    }
}

/// Create a memory pool using smart pointers for efficient allocation/deallocation.
pub struct MemoryPool<T> {
    todo!("Add fields: available: RefCell<Vec<Box<T>>>, allocated: RefCell<Vec<Weak<RefCell<T>>>>")
}

impl<T> MemoryPool<T>
where
    T: Default,
{
    /// Create a new memory pool with initial capacity
    pub fn new(initial_capacity: usize) -> Self {
        todo!("Pre-allocate boxes and store in available list")
    }
    
    /// Allocate an object from the pool
    pub fn allocate(&self) -> Rc<RefCell<T>> {
        todo!("Reuse from pool or create new, track in allocated list")
    }
    
    /// Get statistics about the pool
    pub fn stats(&self) -> (usize, usize) {
        todo!("Return (available_count, allocated_count)")
    }
    
    /// Perform garbage collection - remove deallocated objects from tracking
    pub fn collect(&self) {
        todo!("Remove weak references that can no longer be upgraded")
    }
}

/// Demonstrate a complex data structure: a graph with nodes that can reference each other.
pub struct Graph<T> {
    todo!("Add field: nodes: RefCell<HashMap<usize, Rc<GraphNode<T>>>>")
}

pub struct GraphNode<T> {
    todo!("Add fields: id: usize, value: T, edges: RefCell<Vec<Weak<GraphNode<T>>>>")
}

impl<T> Graph<T> {
    /// Create a new empty graph
    pub fn new() -> Self {
        todo!("Initialize with empty nodes HashMap")
    }
    
    /// Add a node to the graph
    pub fn add_node(&self, id: usize, value: T) -> Rc<GraphNode<T>> {
        todo!("Create node and add to graph")
    }
    
    /// Add an edge between two nodes
    pub fn add_edge(&self, from_id: usize, to_id: usize) -> Result<(), &'static str> {
        todo!("Find nodes and create weak reference edge")
    }
    
    /// Get a node by ID
    pub fn get_node(&self, id: usize) -> Option<Rc<GraphNode<T>>> {
        todo!("Look up node in HashMap and clone Rc")
    }
    
    /// Remove a node and all edges to it
    pub fn remove_node(&self, id: usize) -> Option<Rc<GraphNode<T>>> {
        todo!("Remove from HashMap, edges will be automatically cleaned up by weak refs")
    }
    
    /// Get all node IDs
    pub fn get_node_ids(&self) -> Vec<usize> {
        todo!("Collect all keys from HashMap")
    }
}

impl<T> GraphNode<T> {
    fn new(id: usize, value: T) -> Rc<Self> {
        todo!("Create GraphNode wrapped in Rc")
    }
    
    /// Get connected nodes (that still exist)
    pub fn get_connected_nodes(&self) -> Vec<Rc<GraphNode<T>>> {
        todo!("Upgrade all weak references that are still valid")
    }
    
    /// Get the node's value
    pub fn value(&self) -> &T {
        todo!("Return reference to value")
    }
    
    /// Get the node's ID
    pub fn id(&self) -> usize {
        todo!("Return ID")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_thread_safe_cache_basic() {
        let cache = ThreadSafeCache::new();
        cache.insert("key1".to_string(), 100);
        cache.insert("key2".to_string(), 200);
        
        assert_eq!(cache.get(&"key1".to_string()), Some(100));
        assert_eq!(cache.get(&"key3".to_string()), None);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_thread_safe_cache_threads() {
        let cache = ThreadSafeCache::new();
        let cache_handle = cache.clone_handle();
        
        cache.insert(1, "initial".to_string());
        
        let handle = thread::spawn(move || {
            cache_handle.insert(2, "from_thread".to_string());
            cache_handle.get(&1)
        });
        
        let result = handle.join().unwrap();
        assert_eq!(result, Some("initial".to_string()));
        assert_eq!(cache.get(&2), Some("from_thread".to_string()));
    }

    #[test]
    fn test_thread_safe_cache_get_or_insert() {
        let cache = ThreadSafeCache::new();
        
        let result1 = cache.get_or_insert_with(1, || "computed".to_string());
        assert_eq!(result1, "computed");
        assert_eq!(cache.len(), 1);
        
        let result2 = cache.get_or_insert_with(1, || "should_not_compute".to_string());
        assert_eq!(result2, "computed"); // Should return cached value
    }

    #[test]
    fn test_tree_node_complex_basic() {
        let root = TreeNodeComplex::new("root");
        let child1 = TreeNodeComplex::new("child1");
        let child2 = TreeNodeComplex::new("child2");
        
        TreeNodeComplex::add_child(&root, child1.clone());
        TreeNodeComplex::add_child(&root, child2.clone());
        
        let children_values = root.get_children_values();
        assert_eq!(children_values, vec!["child1", "child2"]);
        
        assert_eq!(child1.get_depth(), 1);
        assert_eq!(root.get_depth(), 0);
    }

    #[test]
    fn test_tree_node_complex_find() {
        let root = TreeNodeComplex::new(1);
        let child1 = TreeNodeComplex::new(2);
        let grandchild = TreeNodeComplex::new(3);
        
        TreeNodeComplex::add_child(&root, child1.clone());
        TreeNodeComplex::add_child(&child1, grandchild.clone());
        
        let found = root.find_node(&3);
        assert!(found.is_some());
        assert_eq!(*found.unwrap().value, 3);
        
        let not_found = root.find_node(&999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_tree_node_complex_path() {
        let root = TreeNodeComplex::new("A");
        let child = TreeNodeComplex::new("B");
        let grandchild = TreeNodeComplex::new("C");
        
        TreeNodeComplex::add_child(&root, child.clone());
        TreeNodeComplex::add_child(&child, grandchild.clone());
        
        let path = grandchild.get_path_from_root();
        assert_eq!(path, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_publisher_subscriber() {
        let publisher = Publisher::new();
        let sub1 = LoggingSubscriber::new("sub1".to_string());
        let sub2 = LoggingSubscriber::new("sub2".to_string());
        
        publisher.subscribe(sub1.clone());
        publisher.subscribe(sub2.clone());
        
        assert_eq!(publisher.active_subscriber_count(), 2);
        
        publisher.publish("Hello".to_string());
        
        let logs1 = sub1.get_logs();
        let logs2 = sub2.get_logs();
        
        assert_eq!(logs1.len(), 1);
        assert_eq!(logs2.len(), 1);
        assert!(logs1[0].contains("Hello"));
        assert!(logs2[0].contains("Hello"));
    }

    #[test]
    fn test_publisher_subscriber_cleanup() {
        let publisher = Publisher::new();
        let sub1 = LoggingSubscriber::new("temp".to_string());
        
        publisher.subscribe(sub1.clone());
        assert_eq!(publisher.active_subscriber_count(), 1);
        
        drop(sub1); // Drop the subscriber
        
        publisher.cleanup_subscribers();
        assert_eq!(publisher.active_subscriber_count(), 0);
    }

    #[test]
    fn test_memory_pool() {
        let pool = MemoryPool::<i32>::new(2);
        let (available, allocated) = pool.stats();
        assert_eq!(available, 2);
        assert_eq!(allocated, 0);
        
        let obj1 = pool.allocate();
        let obj2 = pool.allocate();
        
        *obj1.borrow_mut() = 42;
        *obj2.borrow_mut() = 24;
        
        let (available, allocated) = pool.stats();
        assert!(allocated >= 2); // May have more due to implementation details
        
        drop(obj1);
        pool.collect();
        
        let (available, allocated) = pool.stats();
        assert!(allocated >= 1);
    }

    #[test]
    fn test_graph_basic() {
        let graph = Graph::new();
        
        let node1 = graph.add_node(1, "Node1");
        let node2 = graph.add_node(2, "Node2");
        
        assert!(graph.add_edge(1, 2).is_ok());
        
        let connected = node1.get_connected_nodes();
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].id(), 2);
        
        let node_ids = graph.get_node_ids();
        assert_eq!(node_ids.len(), 2);
        assert!(node_ids.contains(&1));
        assert!(node_ids.contains(&2));
    }

    #[test]
    fn test_graph_edge_cleanup() {
        let graph = Graph::new();
        
        let node1 = graph.add_node(1, "Node1");
        let _node2 = graph.add_node(2, "Node2");
        
        graph.add_edge(1, 2).unwrap();
        
        let connected_before = node1.get_connected_nodes();
        assert_eq!(connected_before.len(), 1);
        
        // Remove node2
        graph.remove_node(2);
        
        let connected_after = node1.get_connected_nodes();
        assert_eq!(connected_after.len(), 0); // Weak reference should be invalid
    }

    #[test]
    fn test_graph_nonexistent_edge() {
        let graph = Graph::new();
        graph.add_node(1, "Node1");
        
        let result = graph.add_edge(1, 999); // Node 999 doesn't exist
        assert!(result.is_err());
    }
}