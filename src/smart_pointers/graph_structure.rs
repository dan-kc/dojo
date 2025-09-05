// Graph Data Structure Practice
//
// Learning objectives:
// - Implementing graph data structures with smart pointers
// - Using HashMap for efficient node lookup
// - Managing bidirectional relationships with Weak<T> references
// - Avoiding memory leaks in cyclic data structures
// - Handling node removal and edge cleanup
// - Understanding graph algorithms and traversals
//
// Run with: cargo test graph_structure

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

/// Demonstrate a complex data structure: a graph with nodes that can reference each other.
pub struct Graph<T> {
    nodes: RefCell<HashMap<usize, Rc<GraphNode<T>>>>,
}

pub struct GraphNode<T> {
    id: usize,
    value: T,
    edges: RefCell<Vec<Weak<GraphNode<T>>>>,
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

    #[test]
    fn test_graph_multiple_edges() {
        let graph = Graph::new();
        
        let node1 = graph.add_node(1, "Hub");
        let _node2 = graph.add_node(2, "A");
        let _node3 = graph.add_node(3, "B");
        let _node4 = graph.add_node(4, "C");
        
        // Create edges from node1 to all others
        assert!(graph.add_edge(1, 2).is_ok());
        assert!(graph.add_edge(1, 3).is_ok());
        assert!(graph.add_edge(1, 4).is_ok());
        
        let connected = node1.get_connected_nodes();
        assert_eq!(connected.len(), 3);
        
        // Verify we can reach all connected nodes
        let connected_ids: Vec<_> = connected.iter().map(|n| n.id()).collect();
        assert!(connected_ids.contains(&2));
        assert!(connected_ids.contains(&3));
        assert!(connected_ids.contains(&4));
    }

    #[test]
    fn test_graph_bidirectional_edges() {
        let graph = Graph::new();
        
        let node1 = graph.add_node(1, "A");
        let node2 = graph.add_node(2, "B");
        
        // Create bidirectional edges
        assert!(graph.add_edge(1, 2).is_ok());
        assert!(graph.add_edge(2, 1).is_ok());
        
        let connected1 = node1.get_connected_nodes();
        let connected2 = node2.get_connected_nodes();
        
        assert_eq!(connected1.len(), 1);
        assert_eq!(connected2.len(), 1);
        assert_eq!(connected1[0].id(), 2);
        assert_eq!(connected2[0].id(), 1);
    }

    #[test]
    fn test_graph_node_removal() {
        let graph = Graph::new();
        
        let node1 = graph.add_node(1, "Persistent");
        let _node2 = graph.add_node(2, "Temporary");
        let _node3 = graph.add_node(3, "Also Temporary");
        
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(1, 3).unwrap();
        graph.add_edge(2, 3).unwrap();
        
        assert_eq!(node1.get_connected_nodes().len(), 2);
        assert_eq!(graph.get_node_ids().len(), 3);
        
        // Remove node 2
        let removed = graph.remove_node(2);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().id(), 2);
        
        // Graph should now have 2 nodes
        assert_eq!(graph.get_node_ids().len(), 2);
        
        // Node1 should have only 1 connection now
        let connected = node1.get_connected_nodes();
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].id(), 3);
    }

    #[test]
    fn test_graph_get_node() {
        let graph = Graph::new();
        
        graph.add_node(1, "Test Node");
        
        let found = graph.get_node(1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().value(), &"Test Node");
        
        let not_found = graph.get_node(999);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_graph_empty_graph() {
        let graph = Graph::<i32>::new();
        
        assert_eq!(graph.get_node_ids().len(), 0);
        assert!(graph.get_node(1).is_none());
        assert!(graph.add_edge(1, 2).is_err());
    }

    #[test]
    fn test_graph_self_edge() {
        let graph = Graph::new();
        
        let node = graph.add_node(1, "Self-referencing");
        
        // Add self-edge
        assert!(graph.add_edge(1, 1).is_ok());
        
        let connected = node.get_connected_nodes();
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].id(), 1);
    }

    #[test]
    fn test_graph_complex_scenario() {
        let graph = Graph::new();
        
        // Create a more complex graph structure
        for i in 1..=5 {
            graph.add_node(i, format!("Node{}", i));
        }
        
        // Create some connections: 1->2, 1->3, 2->4, 3->4, 4->5
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(1, 3).unwrap();
        graph.add_edge(2, 4).unwrap();
        graph.add_edge(3, 4).unwrap();
        graph.add_edge(4, 5).unwrap();
        
        let node1 = graph.get_node(1).unwrap();
        let node4 = graph.get_node(4).unwrap();
        
        assert_eq!(node1.get_connected_nodes().len(), 2); // connects to 2 and 3
        assert_eq!(node4.get_connected_nodes().len(), 1); // connects to 5
        
        // Remove node 4 - this should break connections from 2 and 3 to 4
        graph.remove_node(4);
        
        let node2 = graph.get_node(2).unwrap();
        let node3 = graph.get_node(3).unwrap();
        
        assert_eq!(node2.get_connected_nodes().len(), 0); // no more connection to 4
        assert_eq!(node3.get_connected_nodes().len(), 0); // no more connection to 4
    }

    // Test with numeric values
    #[test]
    fn test_graph_numeric_values() {
        let graph = Graph::new();
        
        let node1 = graph.add_node(1, 100);
        let node2 = graph.add_node(2, 200);
        let node3 = graph.add_node(3, 300);
        
        graph.add_edge(1, 2).unwrap();
        graph.add_edge(2, 3).unwrap();
        
        assert_eq!(node1.value(), &100);
        assert_eq!(node2.value(), &200);
        assert_eq!(node3.value(), &300);
        
        let connected = node1.get_connected_nodes();
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].value(), &200);
    }
}