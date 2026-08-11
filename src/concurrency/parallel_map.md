# Parallel Map Operation

## Solution

```rust
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub fn parallel_map<T, F, R>(
    pool: &ThreadPool,
    items: Vec<T>,
    func: F,
) -> Vec<R>
where
    T: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
    R: Send + 'static,
{
    if items.is_empty() {
        return Vec::new();
    }
    
    let func = Arc::new(func);
    let (tx, rx) = mpsc::channel();
    let items_with_indices: Vec<(usize, T)> = items.into_iter().enumerate().collect();
    let work_queue = Arc::new(Mutex::new(items_with_indices));
    
    let worker_count = pool.worker_count;
    let mut handles = Vec::new();
    
    for _ in 0..worker_count {
        let work_queue = Arc::clone(&work_queue);
        let func = Arc::clone(&func);
        let tx = tx.clone();
        
        let handle = thread::spawn(move || {
            loop {
                let work_item = {
                    let mut queue = work_queue.lock().unwrap();
                    queue.pop()
                };
                
                match work_item {
                    Some((index, item)) => {
                        let result = func(item);
                        tx.send((index, result)).unwrap();
                    }
                    None => break,
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Drop the original sender so the receiver knows when all work is done
    drop(tx);
    
    // Collect all results
    let mut results = Vec::new();
    for (index, result) in rx {
        results.push((index, result));
    }
    
    // Join all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Sort by index to maintain order
    results.sort_by_key(|&(i, _)| i);
    results.into_iter().map(|(_, r)| r).collect()
}
```

```rust
// Alternative non-channel implementation
pub fn parallel_map<T, F, R>(pool: &ThreadPool, items: Vec<T>, func: F) -> Vec<R>
where
    T: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
    R: Send + 'static,
{
    use std::sync::{Arc, Mutex};
    let thread_count = pool.worker_count.min(items.len());
    let queue = Arc::new(Mutex::new(items.into_iter()));
    let func = Arc::new(func);

    let handles: Vec<_> = (0..thread_count)
        .map(|_| {
            let queue = Arc::clone(&queue);
            let func = Arc::clone(&func);
            std::thread::spawn(move || {
                let mut local_res = vec![]; // cap estimate?
                loop {
                    let next = {
                        let mut guard = queue.lock().unwrap();
                        let Some(next) = guard.next() else { break };
                        next
                    };
                    local_res.push(func(next))
                }
                local_res
            })
        })
        .collect();

    let mut res = vec![];
    for handle in handles {
        let processed = handle.join().unwrap();
        res.extend(processed)
    }

    res
}

```

## Explanation

The parallel map implementation distributes work across multiple threads using a shared work queue pattern:

1. **Work Queue**: Items are stored with their indices in a shared `Arc<Mutex<Vec>>` that acts as a work queue.

2. **Worker Threads**: Each worker thread continuously pulls items from the queue, processes them with the provided function, and sends results through a channel.

3. **Order Preservation**: Original indices are maintained with each item, allowing results to be sorted back into the original order after parallel processing.

4. **Synchronization**: The function is wrapped in `Arc` to share it safely across threads, and results are collected through an mpsc channel.

5. **Completion Detection**: Workers exit when the queue is empty, and all threads are joined before returning results.

This pattern efficiently parallelizes computation while maintaining result ordering and thread safety through Rust's ownership and synchronization primitives.
