// cargo test bounded_queue

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
    use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::Duration;

    const BLOCKING_CHECK: Duration = Duration::from_millis(100);
    const COMPLETION_TIMEOUT: Duration = Duration::from_secs(2);

    /// Assert that a worker is still alive but has not completed its queue
    /// operation. A disconnected channel means the worker panicked, which is
    /// different from correctly blocking.
    fn assert_still_blocked<T>(receiver: &Receiver<T>) {
        match receiver.recv_timeout(BLOCKING_CHECK) {
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                panic!("worker exited instead of blocking")
            }
            Ok(_) => panic!("queue operation returned when it should have blocked"),
        }
    }

    #[test]
    fn new_queue_is_empty() {
        let queue = BoundedQueue::<i32>::new(3);

        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn enqueue_and_dequeue_update_length() {
        let queue = BoundedQueue::new(3);

        queue.enqueue(10);
        assert_eq!(queue.len(), 1);

        queue.enqueue(20);
        assert_eq!(queue.len(), 2);

        assert_eq!(queue.dequeue(), 10);
        assert_eq!(queue.len(), 1);

        assert_eq!(queue.dequeue(), 20);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn dequeue_is_fifo() {
        let queue = BoundedQueue::new(4);

        for value in ["first", "second", "third", "fourth"] {
            queue.enqueue(value.to_owned());
        }

        assert_eq!(queue.dequeue(), "first");
        assert_eq!(queue.dequeue(), "second");
        assert_eq!(queue.dequeue(), "third");
        assert_eq!(queue.dequeue(), "fourth");
    }

    #[test]
    fn dequeue_blocks_while_queue_is_empty_then_wakes() {
        let queue = Arc::new(BoundedQueue::new(1));
        let worker_queue = Arc::clone(&queue);
        let (started_tx, started_rx) = mpsc::channel();
        let (completed_tx, completed_rx) = mpsc::channel();

        let worker = thread::spawn(move || {
            started_tx.send(()).unwrap();
            let value = worker_queue.dequeue();
            completed_tx.send(value).unwrap();
        });

        started_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        assert_still_blocked(&completed_rx);

        queue.enqueue(42);
        assert_eq!(completed_rx.recv_timeout(COMPLETION_TIMEOUT), Ok(42));
        worker.join().unwrap();
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn enqueue_blocks_while_queue_is_full_then_wakes() {
        let queue = Arc::new(BoundedQueue::new(1));
        queue.enqueue(10);

        let worker_queue = Arc::clone(&queue);
        let (started_tx, started_rx) = mpsc::channel();
        let (completed_tx, completed_rx) = mpsc::channel();

        let worker = thread::spawn(move || {
            started_tx.send(()).unwrap();
            worker_queue.enqueue(20);
            completed_tx.send(()).unwrap();
        });

        started_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        assert_still_blocked(&completed_rx);
        assert_eq!(queue.len(), 1);

        assert_eq!(queue.dequeue(), 10);
        completed_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        worker.join().unwrap();

        assert_eq!(queue.len(), 1);
        assert_eq!(queue.dequeue(), 20);
    }

    #[test]
    fn freeing_one_slot_allows_only_one_blocked_producer_to_finish() {
        let queue = Arc::new(BoundedQueue::new(1));
        queue.enqueue(0);

        let (started_tx, started_rx) = mpsc::channel();
        let (completed_tx, completed_rx) = mpsc::channel();
        let mut workers = Vec::new();

        for value in [1, 2] {
            let worker_queue = Arc::clone(&queue);
            let started_tx = started_tx.clone();
            let completed_tx = completed_tx.clone();
            workers.push(thread::spawn(move || {
                started_tx.send(()).unwrap();
                worker_queue.enqueue(value);
                completed_tx.send(value).unwrap();
            }));
        }
        drop(started_tx);
        drop(completed_tx);

        for _ in 0..2 {
            started_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        }
        assert_still_blocked(&completed_rx);

        assert_eq!(queue.dequeue(), 0);
        let first = completed_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();

        // The first producer filled the only available slot, so the other
        // producer must still be blocked.
        assert_still_blocked(&completed_rx);
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.dequeue(), first);

        let second = completed_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        assert_ne!(first, second);
        assert_eq!(queue.dequeue(), second);

        for worker in workers {
            worker.join().unwrap();
        }
    }

    #[test]
    fn adding_one_item_allows_only_one_blocked_consumer_to_finish() {
        let queue = Arc::new(BoundedQueue::new(1));
        let (started_tx, started_rx) = mpsc::channel();
        let (completed_tx, completed_rx) = mpsc::channel();
        let mut workers = Vec::new();

        for worker_id in 0..2 {
            let worker_queue = Arc::clone(&queue);
            let started_tx = started_tx.clone();
            let completed_tx = completed_tx.clone();
            workers.push(thread::spawn(move || {
                started_tx.send(()).unwrap();
                let value = worker_queue.dequeue();
                completed_tx.send((worker_id, value)).unwrap();
            }));
        }
        drop(started_tx);
        drop(completed_tx);

        for _ in 0..2 {
            started_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        }
        assert_still_blocked(&completed_rx);

        queue.enqueue(10);
        let first = completed_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        assert_eq!(first.1, 10);

        // Only one item was supplied, so the other consumer must still wait.
        assert_still_blocked(&completed_rx);

        queue.enqueue(20);
        let second = completed_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        assert_eq!(second.1, 20);
        assert_ne!(first.0, second.0);

        for worker in workers {
            worker.join().unwrap();
        }
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn blocked_producer_preserves_fifo_order() {
        let queue = Arc::new(BoundedQueue::new(2));
        queue.enqueue(1);
        queue.enqueue(2);

        let worker_queue = Arc::clone(&queue);
        let (completed_tx, completed_rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            worker_queue.enqueue(3);
            completed_tx.send(()).unwrap();
        });

        assert_still_blocked(&completed_rx);
        assert_eq!(queue.dequeue(), 1);
        completed_rx.recv_timeout(COMPLETION_TIMEOUT).unwrap();
        worker.join().unwrap();

        assert_eq!(queue.dequeue(), 2);
        assert_eq!(queue.dequeue(), 3);
    }

    #[test]
    fn multiple_producers_and_consumers_do_not_lose_or_duplicate_items() {
        const PRODUCERS: usize = 4;
        const CONSUMERS: usize = 4;
        const ITEMS_PER_PRODUCER: usize = 100;
        const ITEMS_PER_CONSUMER: usize = PRODUCERS * ITEMS_PER_PRODUCER / CONSUMERS;

        let queue = Arc::new(BoundedQueue::new(7));
        let start = Arc::new(Barrier::new(PRODUCERS + CONSUMERS + 1));
        let (producer_done_tx, producer_done_rx) = mpsc::channel();
        let (consumer_done_tx, consumer_done_rx) = mpsc::channel();
        let mut producers = Vec::new();
        let mut consumers = Vec::new();

        for producer_id in 0..PRODUCERS {
            let worker_queue = Arc::clone(&queue);
            let worker_start = Arc::clone(&start);
            let producer_done_tx = producer_done_tx.clone();
            producers.push(thread::spawn(move || {
                worker_start.wait();
                for item_id in 0..ITEMS_PER_PRODUCER {
                    worker_queue.enqueue(producer_id * ITEMS_PER_PRODUCER + item_id);
                }
                producer_done_tx.send(()).unwrap();
            }));
        }
        drop(producer_done_tx);

        for _ in 0..CONSUMERS {
            let worker_queue = Arc::clone(&queue);
            let worker_start = Arc::clone(&start);
            let consumer_done_tx = consumer_done_tx.clone();
            consumers.push(thread::spawn(move || {
                worker_start.wait();
                let consumed = (0..ITEMS_PER_CONSUMER)
                    .map(|_| worker_queue.dequeue())
                    .collect::<Vec<_>>();
                consumer_done_tx.send(consumed).unwrap();
            }));
        }
        drop(consumer_done_tx);

        start.wait();

        for _ in 0..PRODUCERS {
            producer_done_rx
                .recv_timeout(COMPLETION_TIMEOUT)
                .expect("producers deadlocked");
        }

        let mut consumed = Vec::with_capacity(PRODUCERS * ITEMS_PER_PRODUCER);
        for _ in 0..CONSUMERS {
            consumed.extend(
                consumer_done_rx
                    .recv_timeout(COMPLETION_TIMEOUT)
                    .expect("consumers deadlocked"),
            );
        }

        for producer in producers {
            producer.join().unwrap();
        }
        for consumer in consumers {
            consumer.join().unwrap();
        }
        consumed.sort_unstable();

        let expected = (0..PRODUCERS * ITEMS_PER_PRODUCER).collect::<Vec<_>>();
        assert_eq!(consumed, expected);
        assert_eq!(queue.len(), 0);
    }
}
