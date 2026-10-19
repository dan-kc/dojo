# smart_pointers/tree_node_complex

Reference implementation. Read after attempting the exercise.

```rust
pub struct TreeNodeComplex<T> {
    value: T,
    children: std::cell::RefCell<Vec<std::rc::Rc<Self>>>,
    parent: std::cell::RefCell<Option<std::rc::Weak<Self>>>,
}
impl<T> TreeNodeComplex<T> {
    pub fn new(value: T) -> std::rc::Rc<Self> {
        std::rc::Rc::new(Self {
            value,
            children: std::cell::RefCell::new(vec![]),
            parent: std::cell::RefCell::new(None),
        })
    }
    pub fn add_child(parent: &std::rc::Rc<Self>, child: std::rc::Rc<Self>) {
        let mut cursor = Some(parent.clone());
        while let Some(node) = cursor {
            assert!(!std::rc::Rc::ptr_eq(&node, &child), "cycle");
            cursor = node
                .parent
                .borrow()
                .as_ref()
                .and_then(std::rc::Weak::upgrade);
        }
        let old = child
            .parent
            .borrow()
            .as_ref()
            .and_then(std::rc::Weak::upgrade);
        if let Some(old) = old {
            if std::rc::Rc::ptr_eq(&old, parent) {
                return;
            }
            old.children
                .borrow_mut()
                .retain(|c| !std::rc::Rc::ptr_eq(c, &child));
        }
        *child.parent.borrow_mut() = Some(std::rc::Rc::downgrade(parent));
        parent.children.borrow_mut().push(child);
    }
    pub fn remove_child(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        let mut children = self.children.borrow_mut();
        let Some(i) = children.iter().position(|c| &c.value == value) else {
            return false;
        };
        let child = children.remove(i);
        *child.parent.borrow_mut() = None;
        true
    }
    pub fn get_children_values(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.children
            .borrow()
            .iter()
            .map(|c| c.value.clone())
            .collect()
    }
    pub fn find_node(self: &std::rc::Rc<Self>, target: &T) -> Option<std::rc::Rc<Self>>
    where
        T: PartialEq,
    {
        if &self.value == target {
            return Some(self.clone());
        }
        self.children
            .borrow()
            .iter()
            .find_map(|c| c.find_node(target))
    }
    pub fn get_depth(&self) -> usize {
        let mut depth = 0;
        let mut next = self
            .parent
            .borrow()
            .as_ref()
            .and_then(std::rc::Weak::upgrade);
        while let Some(node) = next {
            depth += 1;
            next = node
                .parent
                .borrow()
                .as_ref()
                .and_then(std::rc::Weak::upgrade);
        }
        depth
    }
    pub fn get_path_from_root(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut path = vec![self.value.clone()];
        let mut next = self
            .parent
            .borrow()
            .as_ref()
            .and_then(std::rc::Weak::upgrade);
        while let Some(node) = next {
            path.push(node.value.clone());
            next = node
                .parent
                .borrow()
                .as_ref()
                .and_then(std::rc::Weak::upgrade);
        }
        path.reverse();
        path
    }
}
```
