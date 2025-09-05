**Solution:**

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

**Explanation:**

This solution implements a classic bounded blocking queue using condition variables. Key concepts:

1. **Condition Variables**: `Condvar` allows threads to wait for specific conditions (not_empty, not_full)
2. **Wait Loop**: Uses `while` instead of `if` to handle spurious wakeups
3. **Notification**: `notify_one()` wakes up one waiting thread when conditions change
4. **Atomic Operations**: All queue operations happen while holding the mutex

This pattern is fundamental for producer-consumer scenarios with backpressure. The bounded capacity prevents unbounded memory growth, while condition variables ensure efficient blocking without busy-waiting. This is similar to Java's BlockingQueue or Go's buffered channels.