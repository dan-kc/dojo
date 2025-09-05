// Bounded Queue with Blocking Operations Practice
//
// Learning Objectives:
// - Implement thread-safe bounded data structures
// - Handle blocking operations with condition variables
// - Manage capacity constraints in concurrent contexts
//
// cargo test --bin bounded_queue

/// Create a thread-safe message queue with bounded capacity.
/// Multiple producers can enqueue messages, and multiple consumers can dequeue.
/// Implement proper blocking when the queue is full or empty.
struct BoundedQueue<T> {
    data: std::marker::PhantomData<T>, // Define your fields here
}

impl<T> BoundedQueue<T> {
    fn new(capacity: usize) -> Self {
        todo!("Implement new")
    }

    /// Enqueue an item. Blocks if the queue is full.
    fn enqueue(&self, item: T) {
        todo!("Implement enqueue")
    }

    /// Dequeue an item. Blocks if the queue is empty.
    fn dequeue(&self) -> T {
        todo!("Implement dequeue")
    }

    /// Get the current size of the queue.
    fn len(&self) -> usize {
        todo!("Implement len")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_bounded_queue() {
        let queue = Arc::new(BoundedQueue::new(3));
        
        // Test basic enqueue/dequeue
        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.len(), 2);
        
        assert_eq!(queue.dequeue(), 1);
        assert_eq!(queue.len(), 1);
        
        // Test concurrent producers and consumers
        let queue_clone = queue.clone();
        let producer = thread::spawn(move || {
            for i in 3..=5 {
                queue_clone.enqueue(i);
            }
        });
        
        let queue_clone2 = queue.clone();
        let consumer = thread::spawn(move || {
            let mut results = Vec::new();
            for _ in 0..3 {
                results.push(queue_clone2.dequeue());
            }
            results
        });
        
        producer.join().unwrap();
        let consumed = consumer.join().unwrap();
        
        // Should have consumed the remaining items
        assert_eq!(consumed.len(), 3);
        assert!(consumed.contains(&2)); // The item we enqueued earlier
    }
}