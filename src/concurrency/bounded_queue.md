# concurrency/bounded_queue

Reference implementation. Read after attempting the exercise.

```rust
struct BoundedQueue<T> {
    data: std::sync::Arc<(
        std::sync::Mutex<std::collections::VecDeque<T>>,
        std::sync::Condvar,
        std::sync::Condvar,
        usize,
    )>,
}

impl<T> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be positive");
        BoundedQueue {
            data: std::sync::Arc::new((
                std::sync::Mutex::new(std::collections::VecDeque::new()),
                std::sync::Condvar::new(), // not_empty
                std::sync::Condvar::new(), // not_full
                capacity,
            )),
        }
    }

    fn enqueue(&self, item: T) {
        let (queue_mutex, not_empty, not_full, capacity) = &*self.data;

        let mut queue = queue_mutex.lock().unwrap();

        while queue.len() >= *capacity {
            queue = not_full.wait(queue).unwrap();
        }

        queue.push_back(item);

        not_empty.notify_one();
    }

    fn dequeue(&self) -> T {
        let (queue_mutex, not_empty, not_full, _) = &*self.data;

        let mut queue = queue_mutex.lock().unwrap();

        while queue.is_empty() {
            queue = not_empty.wait(queue).unwrap();
        }

        let item = queue.pop_front().unwrap();

        not_full.notify_one();

        item
    }

    fn len(&self) -> usize {
        let (queue_mutex, _, _, _) = &*self.data;

        let queue = queue_mutex.lock().unwrap();

        queue.len()
    }
}
```
