# Thread Pool and Parallel Processing Practice - Solution

## Solution

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn submit<T: Task>(&self, task: T) -> TaskHandle<T::Output> {
        let (tx, rx) = mpsc::channel();
        
        let job = Box::new(move || {
            let result = task.execute();
            let _ = tx.send(result);
        });

        self.sender.as_ref().unwrap().send(job).unwrap();
        
        TaskHandle { receiver: rx }
    }

    pub fn shutdown(mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }

    pub fn active_count(&self) -> usize {
        self.workers.len()
    }
}

pub struct TaskHandle<T> {
    receiver: mpsc::Receiver<T>,
}

impl<T> TaskHandle<T> {
    pub fn get(self) -> T {
        self.receiver.recv().unwrap()
    }

    pub fn try_get(&self) -> Option<T>
    where 
        T: Clone,
    {
        self.receiver.try_recv().ok()
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub trait Task: Send + 'static {
    type Output: Send + 'static;
    fn execute(self) -> Self::Output;
}

pub struct ComputeTask {
    pub input: i32,
}

impl Task for ComputeTask {
    type Output = i32;
    
    fn execute(self) -> Self::Output {
        thread::sleep(std::time::Duration::from_millis(10));
        self.input * self.input
    }
}

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
    let func = Arc::new(func);
    let handles: Vec<_> = items.into_iter().map(|item| {
        let func_clone = Arc::clone(&func);
        
        struct MapTask<T, F> {
            item: T,
            func: Arc<F>,
        }
        
        impl<T, F, R> Task for MapTask<T, F>
        where
            T: Send + 'static,
            F: Fn(T) -> R + Send + Sync + 'static,
            R: Send + 'static,
        {
            type Output = R;
            
            fn execute(self) -> Self::Output {
                (self.func)(self.item)
            }
        }
        
        pool.submit(MapTask { item, func: func_clone })
    }).collect();

    handles.into_iter().map(|handle| handle.get()).collect()
}

pub fn parallel_reduce<T, F>(
    pool: &ThreadPool,
    items: Vec<T>,
    identity: T,
    reduce_fn: F,
) -> T
where
    T: Send + Clone + 'static,
    F: Fn(T, T) -> T + Send + Sync + Copy + 'static,
{
    if items.is_empty() {
        return identity;
    }
    
    if items.len() == 1 {
        return items.into_iter().next().unwrap();
    }

    // Divide into chunks and reduce each chunk
    let chunk_size = std::cmp::max(1, items.len() / pool.active_count());
    let chunks: Vec<_> = items.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect();
    
    struct ReduceTask<T, F> {
        items: Vec<T>,
        identity: T,
        reduce_fn: F,
    }
    
    impl<T, F> Task for ReduceTask<T, F>
    where
        T: Send + Clone + 'static,
        F: Fn(T, T) -> T + Send + Sync + Copy + 'static,
    {
        type Output = T;
        
        fn execute(self) -> Self::Output {
            self.items.into_iter().fold(self.identity, self.reduce_fn)
        }
    }

    let handles: Vec<_> = chunks.into_iter().map(|chunk| {
        pool.submit(ReduceTask {
            items: chunk,
            identity: identity.clone(),
            reduce_fn,
        })
    }).collect();

    let partial_results: Vec<T> = handles.into_iter().map(|handle| handle.get()).collect();
    
    // Combine partial results
    partial_results.into_iter().fold(identity, reduce_fn)
}

pub fn parallel_prime_sieve(pool: &ThreadPool, n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }

    // Simple parallel approach: divide range into chunks and sieve each
    let chunk_size = std::cmp::max(1000, n / pool.active_count());
    let mut all_primes = Vec::new();
    
    struct PrimeTask {
        start: usize,
        end: usize,
    }
    
    impl Task for PrimeTask {
        type Output = Vec<usize>;
        
        fn execute(self) -> Self::Output {
            let mut primes = Vec::new();
            
            for num in self.start..=self.end {
                if num < 2 {
                    continue;
                }
                
                let mut is_prime = true;
                for i in 2..=(num as f64).sqrt() as usize {
                    if num % i == 0 {
                        is_prime = false;
                        break;
                    }
                }
                
                if is_prime {
                    primes.push(num);
                }
            }
            
            primes
        }
    }

    let mut handles = Vec::new();
    for start in (2..=n).step_by(chunk_size) {
        let end = std::cmp::min(start + chunk_size - 1, n);
        handles.push(pool.submit(PrimeTask { start, end }));
    }

    for handle in handles {
        all_primes.extend(handle.get());
    }

    all_primes.sort();
    all_primes
}
```

## Explanation

### Key Concepts Covered:

1. **Thread Pool Architecture**: A fixed number of worker threads that pull jobs from a shared queue, avoiding the overhead of creating/destroying threads for each task.

2. **Job Queue Pattern**: Using `mpsc::channel` to distribute work among worker threads. Jobs are boxed closures that can be sent across threads.

3. **Task Trait**: A generic interface for work that can be executed by the thread pool, allowing type-safe return values.

4. **Worker Management**: Each worker runs in a loop, waiting for jobs from the shared receiver until the channel is closed.

5. **Graceful Shutdown**: Closing the sender channel signals workers to exit, and we join all threads to ensure clean shutdown.

### Important Rust-Specific Considerations:

- **Send and 'static Bounds**: Tasks must be `Send` to transfer between threads and `'static` to outlive the function scope.
- **Arc for Shared Ownership**: Using `Arc` to share the receiver among multiple worker threads.
- **RAII Cleanup**: The thread pool automatically shuts down when dropped, but explicit shutdown is available.
- **Type Safety**: The task system preserves type information for return values.

### Advanced Patterns:

- **Work Stealing**: More advanced implementations could use work-stealing queues for better load balancing.
- **Dynamic Sizing**: Thread pools could dynamically adjust worker count based on load.
- **Priority Queues**: Different priority levels for tasks could be implemented.

### Performance Considerations:

- **Queue Contention**: Single shared queue can become a bottleneck with many workers.
- **Task Granularity**: Very small tasks may have overhead that outweighs parallelism benefits.
- **Memory Allocation**: Boxing tasks has allocation overhead that could be optimized in high-performance scenarios.