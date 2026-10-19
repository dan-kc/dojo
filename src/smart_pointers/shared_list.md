# smart_pointers/shared_list

Reference implementation. Read after attempting the exercise.

```rust
use std::cell::RefCell;

use std::rc::Rc;

#[derive(Debug)]
pub struct SharedList<T> {
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> SharedList<T> {
    pub fn new() -> Self {
        SharedList {
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        SharedList {
            data: Rc::new(RefCell::new(vec)),
        }
    }

    pub fn clone_handle(&self) -> Self {
        SharedList {
            data: Rc::clone(&self.data),
        }
    }

    pub fn push(&self, item: T) {
        self.data.borrow_mut().push(item);
    }

    pub fn pop(&self) -> Option<T> {
        self.data.borrow_mut().pop()
    }

    pub fn len(&self) -> usize {
        self.data.borrow().len()
    }

    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.data.borrow().get(index).cloned()
    }

    pub fn for_each<F>(&self, mut func: F)
    where
        F: FnMut(&T),
    {
        for item in self.data.borrow().iter() {
            func(item);
        }
    }

    pub fn map_in_place<F>(&self, func: F)
    where
        F: Fn(&T) -> T,
        T: Clone,
    {
        let mut data = self.data.borrow_mut();
        for item in data.iter_mut() {
            *item = func(item);
        }
    }
}
```
