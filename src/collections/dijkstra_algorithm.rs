// Dijkstra's Algorithm Practice
//
// Learning Objectives:
// - Use BinaryHeap to implement Dijkstra's shortest path algorithm
// - Practice with priority queues for graph algorithms
// - Understand min-heap operations using Reverse wrapper
// - Apply BinaryHeap for efficient path-finding algorithms
//
// Run with: cargo test --bin dijkstra_algorithm

/// Use BinaryHeap to implement Dijkstra's shortest path algorithm.
/// Return the shortest distances from start node to all other nodes.
pub fn dijkstra_shortest_paths(
    graph: &std::collections::HashMap<usize, Vec<(usize, u32)>>, // node -> [(neighbor, weight)]
    start: usize,
    num_nodes: usize,
) -> Vec<Option<u32>> {
    todo!("Implement Dijkstra's algorithm using BinaryHeap")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_dijkstra_shortest_paths() {
        let mut graph = HashMap::new();
        graph.insert(0, vec![(1, 4), (2, 1)]);
        graph.insert(1, vec![(3, 1)]);
        graph.insert(2, vec![(1, 2), (3, 5)]);
        graph.insert(3, vec![]);
        
        let distances = dijkstra_shortest_paths(&graph, 0, 4);
        
        assert_eq!(distances[0], Some(0));  // Distance to self
        assert_eq!(distances[1], Some(3));  // 0->2->1 (cost 1+2=3) is shorter than 0->1 (cost 4)
        assert_eq!(distances[2], Some(1));  // Direct edge 0->2
        assert_eq!(distances[3], Some(4));  // 0->2->1->3 (cost 1+2+1=4)
    }
}