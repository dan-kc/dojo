# smart_pointers/graph_structure

Reference implementation. Read after attempting the exercise.

```rust
pub struct Graph<T> {
    nodes: std::cell::RefCell<std::collections::HashMap<usize, std::rc::Rc<GraphNode<T>>>>,
}
pub struct GraphNode<T> {
    id: usize,
    value: T,
    edges: std::cell::RefCell<Vec<std::rc::Weak<GraphNode<T>>>>,
}
impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            nodes: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }
    pub fn add_node(&self, id: usize, value: T) -> std::rc::Rc<GraphNode<T>> {
        let mut nodes = self.nodes.borrow_mut();
        assert!(!nodes.contains_key(&id));
        let node = std::rc::Rc::new(GraphNode {
            id,
            value,
            edges: std::cell::RefCell::new(vec![]),
        });
        nodes.insert(id, node.clone());
        node
    }
    pub fn add_edge(&self, from_id: usize, to_id: usize) -> Result<(), &'static str> {
        let nodes = self.nodes.borrow();
        let from = nodes.get(&from_id).ok_or("missing source")?;
        let to = nodes.get(&to_id).ok_or("missing target")?;
        let mut edges = from.edges.borrow_mut();
        if !edges
            .iter()
            .any(|edge| edge.upgrade().is_some_and(|n| std::rc::Rc::ptr_eq(&n, to)))
        {
            edges.push(std::rc::Rc::downgrade(to));
        }
        Ok(())
    }
    pub fn get_node(&self, id: usize) -> Option<std::rc::Rc<GraphNode<T>>> {
        self.nodes.borrow().get(&id).cloned()
    }
    pub fn remove_node(&self, id: usize) -> Option<std::rc::Rc<GraphNode<T>>> {
        let mut nodes = self.nodes.borrow_mut();
        let removed = nodes.remove(&id)?;
        for node in nodes.values() {
            node.edges.borrow_mut().retain(|edge| {
                edge.upgrade()
                    .is_some_and(|n| !std::rc::Rc::ptr_eq(&n, &removed))
            });
        }
        removed.edges.borrow_mut().clear();
        Some(removed)
    }
    pub fn get_node_ids(&self) -> Vec<usize> {
        let mut ids: Vec<_> = self.nodes.borrow().keys().copied().collect();
        ids.sort_unstable();
        ids
    }
}
impl<T> GraphNode<T> {
    pub fn get_connected_nodes(&self) -> Vec<std::rc::Rc<Self>> {
        self.edges
            .borrow()
            .iter()
            .filter_map(std::rc::Weak::upgrade)
            .collect()
    }
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn id(&self) -> usize {
        self.id
    }
}
```
