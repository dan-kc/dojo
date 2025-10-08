// cargo test find_connected_components

/// Find connected components in a graph represented as edges.
/// Use HashSet to track visited nodes and implement depth-first search.
pub fn find_connected_components(
    #[allow(unused_variables)] edges: Vec<(i32, i32)>,
) -> Vec<std::collections::HashSet<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_find_connected_components() {
        let edges = vec![
            (1, 2),
            (2, 3), // Component 1: {1, 2, 3}
            (4, 5), // Component 2: {4, 5}
            (6, 7),
            (7, 8),
            (8, 6), // Component 3: {6, 7, 8}
        ];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 3);

        // Check that all nodes are covered
        let all_nodes: HashSet<i32> = components
            .iter()
            .flat_map(|comp| comp.iter())
            .cloned()
            .collect();
        let expected_nodes: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8].iter().cloned().collect();
        assert_eq!(all_nodes, expected_nodes);

        // Check component sizes
        let sizes: Vec<usize> = components.iter().map(|comp| comp.len()).collect();
        let mut sizes = sizes;
        sizes.sort();
        assert_eq!(sizes, vec![2, 3, 3]);
    }

    #[test]
    fn test_connected_components_single_component() {
        let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5)];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 1);

        let expected_nodes: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        assert_eq!(components[0], expected_nodes);
    }

    #[test]
    fn test_connected_components_no_edges() {
        let edges = vec![];
        let components = find_connected_components(edges);
        assert!(components.is_empty());
    }

    #[test]
    fn test_connected_components_isolated_nodes() {
        // Each edge connects two nodes, but no edges connect components
        let edges = vec![(1, 2), (3, 4), (5, 6)];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 3);

        // Each component should have exactly 2 nodes
        for component in &components {
            assert_eq!(component.len(), 2);
        }

        // Check specific components exist
        let comp1: HashSet<i32> = [1, 2].iter().cloned().collect();
        let comp2: HashSet<i32> = [3, 4].iter().cloned().collect();
        let comp3: HashSet<i32> = [5, 6].iter().cloned().collect();

        assert!(components.contains(&comp1));
        assert!(components.contains(&comp2));
        assert!(components.contains(&comp3));
    }

    #[test]
    fn test_connected_components_self_loop() {
        let edges = vec![
            (1, 1), // Self loop
            (2, 3),
        ];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 2);

        // Self-loop should create a component with just one node
        let single_node: HashSet<i32> = [1].iter().cloned().collect();
        let pair_nodes: HashSet<i32> = [2, 3].iter().cloned().collect();

        assert!(components.contains(&single_node));
        assert!(components.contains(&pair_nodes));
    }

    #[test]
    fn test_connected_components_duplicate_edges() {
        let edges = vec![
            (1, 2),
            (1, 2), // Duplicate
            (2, 3),
            (4, 5),
        ];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 2);

        let comp1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let comp2: HashSet<i32> = [4, 5].iter().cloned().collect();

        assert!(components.contains(&comp1));
        assert!(components.contains(&comp2));
    }

    #[test]
    fn test_connected_components_bidirectional() {
        let edges = vec![
            (1, 2),
            (2, 1), // Reverse direction (should not affect result)
            (3, 4),
        ];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 2);

        let comp1: HashSet<i32> = [1, 2].iter().cloned().collect();
        let comp2: HashSet<i32> = [3, 4].iter().cloned().collect();

        assert!(components.contains(&comp1));
        assert!(components.contains(&comp2));
    }

    #[test]
    fn test_connected_components_complex_graph() {
        // Create a more complex graph with cycles and multiple paths
        let edges = vec![
            (1, 2),
            (2, 3),
            (3, 1), // Triangle: {1, 2, 3}
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (4, 6), // Complex component: {4, 5, 6, 7}
            (8, 9), // Simple pair: {8, 9}
        ];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 3);

        // Check specific components
        let triangle: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let complex: HashSet<i32> = [4, 5, 6, 7].iter().cloned().collect();
        let pair: HashSet<i32> = [8, 9].iter().cloned().collect();

        assert!(components.contains(&triangle));
        assert!(components.contains(&complex));
        assert!(components.contains(&pair));
    }

    #[test]
    fn test_connected_components_negative_nodes() {
        let edges = vec![(-1, -2), (-2, -3), (1, 2)];

        let components = find_connected_components(edges);
        assert_eq!(components.len(), 2);

        let negative_comp: HashSet<i32> = [-1, -2, -3].iter().cloned().collect();
        let positive_comp: HashSet<i32> = [1, 2].iter().cloned().collect();

        assert!(components.contains(&negative_comp));
        assert!(components.contains(&positive_comp));
    }
}
