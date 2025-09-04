// Box<T> Basics Practice
//
// Learning objectives:
// - Understanding heap allocation with Box<T>
// - Recursive data structures
// - When to use Box<T> vs stack allocation
// - Moving data between stack and heap
//
// cargo test --lib smart_pointers::box_basics

/// Create a binary tree node using Box<T> for recursive structure.
/// Each node contains a value and optional left and right children.
#[derive(Debug, PartialEq)]
pub struct TreeNode<T> {
    todo!("Define fields: value: T, left: Option<Box<TreeNode<T>>>, right: Option<Box<TreeNode<T>>>")
}

impl<T> TreeNode<T> {
    /// Create a new leaf node (no children)
    pub fn new(value: T) -> Self {
        todo!("Create node with value and no children")
    }
    
    /// Create a new node with left and right children
    pub fn with_children(value: T, left: TreeNode<T>, right: TreeNode<T>) -> Self {
        todo!("Create node with value and both children boxed")
    }
    
    /// Add a left child to this node
    pub fn add_left(&mut self, child: TreeNode<T>) {
        todo!("Set left child, wrapping in Box")
    }
    
    /// Add a right child to this node
    pub fn add_right(&mut self, child: TreeNode<T>) {
        todo!("Set right child, wrapping in Box")
    }
    
    /// Calculate the depth of the tree (max distance from root to leaf)
    pub fn depth(&self) -> usize {
        todo!("Recursively calculate maximum depth")
    }
    
    /// Count total nodes in the tree
    pub fn count_nodes(&self) -> usize {
        todo!("Recursively count all nodes")
    }
    
    /// Collect all values in the tree using in-order traversal
    pub fn in_order_values(&self) -> Vec<&T> {
        todo!("Traverse left, visit node, traverse right")
    }
}

/// Implement a simple linked list using Box<T> for the next pointer.
#[derive(Debug)]
pub struct LinkedList<T> {
    todo!("Define field: head: Option<Box<ListNode<T>>>")
}

#[derive(Debug)]
struct ListNode<T> {
    todo!("Define fields: data: T, next: Option<Box<ListNode<T>>>")
}

impl<T> LinkedList<T> {
    /// Create a new empty linked list
    pub fn new() -> Self {
        todo!("Create empty list")
    }
    
    /// Add an element to the front of the list
    pub fn push_front(&mut self, data: T) {
        todo!("Create new node and make it the head")
    }
    
    /// Remove and return the front element
    pub fn pop_front(&mut self) -> Option<T> {
        todo!("Remove head node and return its data")
    }
    
    /// Get a reference to the front element without removing it
    pub fn peek_front(&self) -> Option<&T> {
        todo!("Return reference to head data if present")
    }
    
    /// Calculate the length of the list
    pub fn len(&self) -> usize {
        todo!("Iterate through list counting nodes")
    }
    
    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        todo!("Check if head is None")
    }
}

/// Create a function that demonstrates when to use Box<T> vs stack allocation.
/// This function takes a large struct and decides whether to allocate on heap or stack
/// based on the size criteria.
pub fn smart_allocation<T>(data: T, force_heap: bool) -> either::Either<T, Box<T>>
where
    T: Clone,
{
    todo!("Return Left(data) for stack allocation or Right(Box::new(data)) for heap allocation")
}

/// Implement a function that converts a Vec<T> to a binary tree.
/// The tree should be balanced (or as close to balanced as possible).
pub fn vec_to_balanced_tree<T>(mut vec: Vec<T>) -> Option<TreeNode<T>> {
    todo!("Convert vector to balanced binary tree recursively")
}

/// Create a function that demonstrates Box<T> deref coercion.
/// This function should work with both T and Box<T> through deref coercion.
pub fn use_deref_coercion<T>(boxed_value: Box<T>) -> T
where
    T: Clone,
{
    todo!("Use deref coercion to access the value and clone it")
}

/// Implement a function that creates a chain of boxed values.
/// Each box contains the previous box, creating a nested structure.
pub fn create_nested_boxes(depth: usize, initial_value: i32) -> Box<i32> {
    todo!("Create nested boxes, each containing the next level")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_creation() {
        let leaf = TreeNode::new(42);
        assert_eq!(leaf.value, 42);
        assert!(leaf.left.is_none());
        assert!(leaf.right.is_none());
    }

    #[test]
    fn test_tree_with_children() {
        let left = TreeNode::new(1);
        let right = TreeNode::new(3);
        let root = TreeNode::with_children(2, left, right);
        
        assert_eq!(root.value, 2);
        assert_eq!(root.left.as_ref().unwrap().value, 1);
        assert_eq!(root.right.as_ref().unwrap().value, 3);
    }

    #[test]
    fn test_tree_add_children() {
        let mut root = TreeNode::new(5);
        root.add_left(TreeNode::new(3));
        root.add_right(TreeNode::new(7));
        
        assert_eq!(root.left.as_ref().unwrap().value, 3);
        assert_eq!(root.right.as_ref().unwrap().value, 7);
    }

    #[test]
    fn test_tree_depth() {
        let mut root = TreeNode::new(1);
        assert_eq!(root.depth(), 0);
        
        root.add_left(TreeNode::new(2));
        assert_eq!(root.depth(), 1);
        
        let mut left_child = TreeNode::new(3);
        left_child.add_left(TreeNode::new(4));
        root.add_left(left_child);
        assert_eq!(root.depth(), 2);
    }

    #[test]
    fn test_tree_count_nodes() {
        let left = TreeNode::new(1);
        let right = TreeNode::new(3);
        let root = TreeNode::with_children(2, left, right);
        
        assert_eq!(root.count_nodes(), 3);
        
        let single_node = TreeNode::new(42);
        assert_eq!(single_node.count_nodes(), 1);
    }

    #[test]
    fn test_tree_in_order_traversal() {
        let left = TreeNode::new(1);
        let right = TreeNode::new(3);
        let root = TreeNode::with_children(2, left, right);
        
        let values = root.in_order_values();
        assert_eq!(values, vec![&1, &2, &3]);
    }

    #[test]
    fn test_linked_list_creation() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_linked_list_push_pop() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_linked_list_peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek_front(), None);
        
        list.push_front(42);
        assert_eq!(list.peek_front(), Some(&42));
        assert_eq!(list.len(), 1); // peek shouldn't remove element
        
        list.push_front(24);
        assert_eq!(list.peek_front(), Some(&24));
    }

    #[test]
    fn test_smart_allocation() {
        let small_data = 42i32;
        
        // Force heap allocation
        let heap_result = smart_allocation(small_data, true);
        assert!(heap_result.is_right());
        
        // Allow stack allocation  
        let stack_result = smart_allocation(small_data, false);
        // Implementation can choose stack or heap based on criteria
    }

    #[test]
    fn test_vec_to_balanced_tree() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let tree = vec_to_balanced_tree(vec);
        
        assert!(tree.is_some());
        let root = tree.unwrap();
        assert_eq!(root.count_nodes(), 7);
        
        // Test empty vector
        let empty: Vec<i32> = vec![];
        assert!(vec_to_balanced_tree(empty).is_none());
    }

    #[test]
    fn test_vec_to_balanced_tree_single_element() {
        let vec = vec![42];
        let tree = vec_to_balanced_tree(vec).unwrap();
        assert_eq!(tree.value, 42);
        assert!(tree.left.is_none());
        assert!(tree.right.is_none());
    }

    #[test]
    fn test_use_deref_coercion() {
        let boxed = Box::new(String::from("hello"));
        let cloned = use_deref_coercion(boxed);
        assert_eq!(cloned, "hello");
    }

    #[test]
    fn test_create_nested_boxes() {
        let nested = create_nested_boxes(3, 42);
        assert_eq!(*nested, 42);
        
        let single = create_nested_boxes(1, 100);
        assert_eq!(*single, 100);
        
        let zero_depth = create_nested_boxes(0, 0);
        assert_eq!(*zero_depth, 0);
    }
}