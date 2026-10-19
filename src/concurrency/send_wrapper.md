# concurrency/send_wrapper

Reference implementation. Read after attempting the exercise.

```rust
pub struct SendWrapper<T> {
    value: std::sync::Mutex<T>,
}
impl<T> SendWrapper<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: std::sync::Mutex::new(value),
        }
    }
    pub fn with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        f(&*self.value.lock().unwrap())
    }
    pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        f(&mut *self.value.lock().unwrap())
    }
}
```
