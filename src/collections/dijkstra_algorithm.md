# Dijkstra's Algorithm Solution

## Implementation

```rust
pub fn dijkstra_shortest_paths(
    graph: &std::collections::HashMap<usize, Vec<(usize, u32)>>, // node -> [(neighbor, weight)]
    start: usize,
    num_nodes: usize,
) -> Vec<Option<u32>> {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    
    // Initialize distances with None (infinite distance)
    let mut distances = vec![None; num_nodes];
    distances[start] = Some(0);
    
    // Min-heap using Reverse wrapper: (distance, node)
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u32, start)));
    
    while let Some(Reverse((current_dist, current_node))) = heap.pop() {
        // Skip if we've already found a shorter path
        if let Some(recorded_dist) = distances[current_node] {
            if current_dist > recorded_dist {
                continue;
            }
        }
        
        // Explore neighbors
        if let Some(neighbors) = graph.get(&current_node) {
            for &(neighbor, edge_weight) in neighbors {
                let new_distance = current_dist + edge_weight;
                
                // Update distance if we found a shorter path
                let should_update = match distances[neighbor] {
                    None => true,
                    Some(existing_dist) => new_distance < existing_dist,
                };
                
                if should_update {
                    distances[neighbor] = Some(new_distance);
                    heap.push(Reverse((new_distance, neighbor)));
                }
            }
        }
    }
    
    distances
}
```

## Explanation

This solution implements Dijkstra's shortest path algorithm using BinaryHeap:

1. **Initialization**: Set distances to None (infinite), start node to 0
2. **Min-heap**: Use `Reverse` wrapper to convert max-heap to min-heap behavior
3. **Priority processing**: Always process the node with minimum distance first
4. **Relaxation**: Update distances when shorter paths are found
5. **Early termination**: Skip nodes already processed with better distances

The algorithm guarantees finding shortest paths from the start node to all reachable nodes.

## Key Learning Points

- **BinaryHeap usage**: Max-heap by default, use Reverse for min-heap behavior
- **Dijkstra's algorithm**: Greedy approach to shortest path problems
- **Priority queue patterns**: Processing items by priority order
- **Graph representation**: Using HashMap for adjacency list representation

## Rust Concepts Demonstrated

- BinaryHeap for priority queue operations
- `std::cmp::Reverse` for heap ordering control
- Option types for representing infinite distances
- Pattern matching with destructuring
- HashMap navigation and graph algorithms