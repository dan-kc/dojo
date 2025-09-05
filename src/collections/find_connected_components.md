# Graph Connected Components Solution

## Implementation

```rust
pub fn find_connected_components(
    edges: Vec<(i32, i32)>,
) -> Vec<std::collections::HashSet<i32>> {
    use std::collections::{HashMap, HashSet};
    
    // Build adjacency list representation
    let mut graph = HashMap::new();
    for (u, v) in edges {
        graph.entry(u).or_insert_with(Vec::new).push(v);
        graph.entry(v).or_insert_with(Vec::new).push(u);
    }
    
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    // DFS for each unvisited node
    for &node in graph.keys() {
        if !visited.contains(&node) {
            let mut component = HashSet::new();
            dfs(&graph, node, &mut visited, &mut component);
            components.push(component);
        }
    }
    
    components
}

fn dfs(
    graph: &std::collections::HashMap<i32, Vec<i32>>,
    node: i32,
    visited: &mut std::collections::HashSet<i32>,
    component: &mut std::collections::HashSet<i32>,
) {
    visited.insert(node);
    component.insert(node);
    
    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if !visited.contains(&neighbor) {
                dfs(graph, neighbor, visited, component);
            }
        }
    }
}
```

## Alternative Implementation (Iterative DFS)

```rust
pub fn find_connected_components(
    edges: Vec<(i32, i32)>,
) -> Vec<std::collections::HashSet<i32>> {
    use std::collections::{HashMap, HashSet};
    
    let mut graph = HashMap::new();
    for (u, v) in edges {
        graph.entry(u).or_insert_with(Vec::new).push(v);
        graph.entry(v).or_insert_with(Vec::new).push(u);
    }
    
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    for &node in graph.keys() {
        if !visited.contains(&node) {
            let mut component = HashSet::new();
            let mut stack = vec![node];
            
            while let Some(current) = stack.pop() {
                if visited.insert(current) {
                    component.insert(current);
                    
                    if let Some(neighbors) = graph.get(&current) {
                        for &neighbor in neighbors {
                            if !visited.contains(&neighbor) {
                                stack.push(neighbor);
                            }
                        }
                    }
                }
            }
            
            components.push(component);
        }
    }
    
    components
}
```

## Explanation

This solution finds connected components in an undirected graph:

1. **Graph representation**: Builds adjacency list from edge pairs
2. **DFS traversal**: Uses depth-first search to explore connected nodes
3. **Visit tracking**: HashSet prevents revisiting nodes
4. **Component isolation**: Each DFS run identifies one connected component

## Key Learning Points

- **Graph algorithms**: Classical connected components problem
- **DFS implementation**: Both recursive and iterative approaches
- **HashSet for visited tracking**: Efficient O(1) membership testing
- **Adjacency list**: Efficient graph representation using HashMap

## Algorithm Complexity

- **Time**: O(V + E) where V is vertices and E is edges
- **Space**: O(V) for visited set and recursion stack
- **Graph building**: O(E) to construct adjacency list

## Use Cases

- **Social networks**: Finding friend circles or communities
- **Network analysis**: Identifying isolated network segments
- **Image processing**: Finding connected regions in binary images
- **Clustering**: Grouping related data points

## Rust Concepts Demonstrated

- HashMap for graph representation
- HashSet for efficient membership testing
- Recursive algorithms with mutable references
- Pattern matching with `if let` for optional values
- Collection building and manipulation patterns