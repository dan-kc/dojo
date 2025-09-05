// Complex Tree Structure with Parent-Child Relationships Practice
//
// Learning objectives:
// - Using Rc<T> for shared ownership of tree nodes
// - Using RefCell<T> for interior mutability in tree structures
// - Using Weak<T> references to break cycles (child -> parent)
// - Implementing complex tree traversals and operations
// - Managing bidirectional relationships safely
//
// Run with: cargo test tree_node_complex

use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Create a complex tree structure with parent-child relationships using multiple smart pointers.
/// Parents hold strong references to children, children hold weak references to parents.
pub struct TreeNodeComplex<T> {
    value: T,
    children: RefCell<Vec<Rc<TreeNodeComplex<T>>>>,
    parent: RefCell<Option<Weak<TreeNodeComplex<T>>>>,
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(found.unwrap().value, 3);
        
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
    fn test_tree_node_remove_child() {
        let root = TreeNodeComplex::new("root");
        let child1 = TreeNodeComplex::new("child1");
        let child2 = TreeNodeComplex::new("child2");
        
        TreeNodeComplex::add_child(&root, child1.clone());
        TreeNodeComplex::add_child(&root, child2.clone());
        
        assert_eq!(root.get_children_values().len(), 2);
        
        let removed = root.remove_child(&"child1");
        assert!(removed);
        
        let children_after = root.get_children_values();
        assert_eq!(children_after, vec!["child2"]);
        
        // Child should no longer have parent
        assert_eq!(child1.get_depth(), 0);
    }

    #[test]
    fn test_tree_node_multi_level_depth() {
        let root = TreeNodeComplex::new(0);
        let level1 = TreeNodeComplex::new(1);
        let level2 = TreeNodeComplex::new(2);
        let level3 = TreeNodeComplex::new(3);
        
        TreeNodeComplex::add_child(&root, level1.clone());
        TreeNodeComplex::add_child(&level1, level2.clone());
        TreeNodeComplex::add_child(&level2, level3.clone());
        
        assert_eq!(root.get_depth(), 0);
        assert_eq!(level1.get_depth(), 1);
        assert_eq!(level2.get_depth(), 2);
        assert_eq!(level3.get_depth(), 3);
    }

    #[test]
    fn test_tree_node_cycle_prevention() {
        let root = TreeNodeComplex::new("root");
        let child = TreeNodeComplex::new("child");
        
        TreeNodeComplex::add_child(&root, child.clone());
        
        // Child should have weak reference to parent
        assert_eq!(child.get_depth(), 1);
        
        // Path should work correctly
        let path = child.get_path_from_root();
        assert_eq!(path, vec!["root", "child"]);
    }
}