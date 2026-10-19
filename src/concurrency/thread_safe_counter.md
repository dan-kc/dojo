# concurrency/thread_safe_counter

Reference implementation. Read after attempting the exercise.

```rust
use std::sync::{Arc, Mutex};

use std::thread;

pub struct ThreadSafeCounter {
    count: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    pub fn new(initial: i32) -> Self {
        Self {
            count: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    pub fn get(&self) -> i32 {
        let count = self.count.lock().unwrap();
        *count
    }

    pub fn add(&self, value: i32) {
        let mut count = self.count.lock().unwrap();
        *count += value;
    }
}

pub fn share_across_threads<T: Send + Sync + 'static>(value: T) -> Arc<T> {
    Arc::new(value)
}

/// Test helper to verify Send + Sync requirements at compile time
fn assert_send<T: Send>() {}

fn assert_sync<T: Sync>() {}

fn assert_send_sync<T: Send + Sync>() {}
```
