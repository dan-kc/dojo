# collections/find_connected_components

Reference implementation. Read after attempting the exercise.

```rust
pub fn find_connected_components(edges: Vec<(i32, i32)>) -> Vec<std::collections::HashSet<i32>> {
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
