# smart_pointers/buffer_pool

Reference implementation. Read after attempting the exercise.

```rust
pub struct BufferPool {
    available: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
    capacity: usize,
}
pub struct BufferLease {
    pool: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
    buffer: Option<Vec<u8>>,
}
impl BufferPool {
    pub fn new(count: usize, capacity: usize) -> Self {
        Self {
            available: std::rc::Rc::new(std::cell::RefCell::new(
                (0..count).map(|_| Vec::with_capacity(capacity)).collect(),
            )),
            capacity,
        }
    }
    pub fn acquire(&self) -> BufferLease {
        let buffer = self
            .available
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(self.capacity));
        BufferLease {
            pool: self.available.clone(),
            buffer: Some(buffer),
        }
    }
    pub fn available(&self) -> usize {
        self.available.borrow().len()
    }
}
impl std::ops::Deref for BufferLease {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        self.buffer.as_ref().unwrap()
    }
}
impl std::ops::DerefMut for BufferLease {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        self.buffer.as_mut().unwrap()
    }
}
impl Drop for BufferLease {
    fn drop(&mut self) {
        if let Some(mut buffer) = self.buffer.take() {
            buffer.clear();
            self.pool.borrow_mut().push(buffer);
        }
    }
}
```
