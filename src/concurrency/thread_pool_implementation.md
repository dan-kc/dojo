# Thread Pool Implementation

## Solution

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
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

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }

    pub fn submit<T: Task>(&self, task: T) -> TaskHandle<T::Output> {
        let (tx, rx) = mpsc::channel();
        
        let job = Box::new(move || {
            let result = task.execute();
            tx.send(result).ok();
        });
        
        self.sender.send(Message::NewJob(job)).unwrap();
        
        TaskHandle {
            receiver: Some(rx),
        }
    }

    pub fn shutdown(self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        for worker in self.workers {
            if let Some(thread) = worker.thread {
                thread.join().unwrap();
            }
        }
    }

    pub fn active_count(&self) -> usize {
        self.workers.len()
    }
}

pub struct TaskHandle<T> {
    receiver: Option<mpsc::Receiver<T>>,
}

impl<T> TaskHandle<T> {
    pub fn get(mut self) -> T {
        self.receiver.take().unwrap().recv().unwrap()
    }

    pub fn try_get(&self) -> Option<T>
    where 
        T: Clone,
    {
        self.receiver.as_ref()?.try_recv().ok()
    }
}
```

## Explanation

The thread pool implementation uses a channel-based architecture where worker threads wait for jobs sent through a shared channel. Key concepts include:

1. **Worker Threads**: Each worker runs in its own thread, continuously waiting for jobs from a shared receiver protected by `Arc<Mutex<>>`.

2. **Message Passing**: Uses an enum `Message` to handle both job execution and shutdown signals cleanly.

3. **Task Submission**: When a task is submitted, it's wrapped in a closure that executes the task and sends results through a dedicated channel.

4. **Result Handling**: `TaskHandle` holds a receiver channel to retrieve task results, supporting both blocking `get()` and non-blocking `try_get()`.

5. **Graceful Shutdown**: Sends terminate messages to all workers and joins threads to ensure clean shutdown.

This pattern demonstrates Rust's ownership system working with concurrent execution, using channels for safe communication between threads.