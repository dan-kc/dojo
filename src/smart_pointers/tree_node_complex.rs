// cargo test --lib smart_pointers::tree_node_complex::

//! Build an acyclic tree with strongly owned children and non-owning parent links.
//! Adding a child already attached to this parent is a no-op. Reparenting detaches it from the previous parent.
//! Reject a self/ancestor attachment by panicking before changing either tree. Remove only the first child with a matching value.
//! Search depth-first in insertion order, including the receiver itself, returning the same shared node handle.
//! Depth/path follow live parents; a dropped parent makes its surviving child a root.

pub struct TreeNodeComplex<T> {
    value: T,
    children: std::cell::RefCell<Vec<std::rc::Rc<Self>>>,
    parent: std::cell::RefCell<Option<std::rc::Weak<Self>>>,
}
impl<T> TreeNodeComplex<T> {
    pub fn new(value: T) -> std::rc::Rc<Self> {
        todo!()
    }
    pub fn add_child(parent: &std::rc::Rc<Self>, child: std::rc::Rc<Self>) {
        todo!()
    }
    pub fn remove_child(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        todo!()
    }
    pub fn get_children_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        todo!()
    }
    pub fn find_node(self: &std::rc::Rc<Self>, target: &T) -> Option<std::rc::Rc<Self>>
    where
        T: PartialEq,
    {
        todo!()
    }
    pub fn get_depth(&self) -> usize {
        todo!()
    }
    pub fn get_path_from_root(&self) -> Vec<T>
    where
        T: Clone,
    {
        todo!()
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
#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    #[test]
    fn root_identity_and_reparenting() {
        let a = TreeNodeComplex::new(1);
        let b = TreeNodeComplex::new(2);
        let c = TreeNodeComplex::new(3);
        assert!(std::rc::Rc::ptr_eq(&a, &a.find_node(&1).unwrap()));
        TreeNodeComplex::add_child(&a, c.clone());
        TreeNodeComplex::add_child(&b, c.clone());
        assert!(a.get_children_values().is_empty());
        assert_eq!(c.get_path_from_root(), [2, 3]);
        drop(b);
        assert_eq!(c.get_depth(), 0);
    }
    #[test]
    #[should_panic(expected = "cycle")]
    fn rejects_ancestor() {
        let a = TreeNodeComplex::new(1);
        let b = TreeNodeComplex::new(2);
        TreeNodeComplex::add_child(&a, b.clone());
        TreeNodeComplex::add_child(&b, a);
    }
}
