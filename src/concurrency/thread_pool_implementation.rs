// Thread Pool Implementation Practice
//
// Learning Objectives:
// - Implement a basic thread pool for task execution
// - Understand work stealing and load balancing
// - Practice parallel processing patterns
// - Handle thread pool shutdown gracefully
//
// cargo test --bin thread_pool_implementation

/// A simple thread pool implementation that can execute tasks concurrently.
pub struct ThreadPool {
    data: (), // Define your fields here
}

/// Trait for tasks that can be executed by the thread pool
pub trait Task: Send + 'static {
    type Output: Send + 'static;
    fn execute(self) -> Self::Output;
}

impl ThreadPool {
    /// Create a new thread pool with the specified number of worker threads.
    pub fn new(size: usize) -> ThreadPool {
        todo!("Implement ThreadPool::new")
    }

    /// Submit a task for execution. Returns a handle to retrieve the result.
    pub fn submit<T: Task>(&self, task: T) -> TaskHandle<T::Output> {
        todo!("Implement submit")
    }

    /// Gracefully shutdown the thread pool, waiting for all tasks to complete.
    pub fn shutdown(self) {
        todo!("Implement shutdown")
    }

    /// Get the number of active worker threads.
    pub fn active_count(&self) -> usize {
        todo!("Implement active_count")
    }
}

/// Handle to retrieve the result of an executed task.
pub struct TaskHandle<T> {
    data: std::marker::PhantomData<T>, // Define your fields here
}

impl<T> TaskHandle<T> {
    /// Block until the task completes and return its result.
    pub fn get(self) -> T {
        todo!("Implement get")
    }

    /// Try to get the result without blocking. Returns None if not ready.
    pub fn try_get(&self) -> Option<T>
    where 
        T: Clone,
    {
        todo!("Implement try_get")
    }
}

/// Simple task implementation for testing
pub struct ComputeTask {
    pub input: i32,
}

impl Task for ComputeTask {
    type Output = i32;
    
    fn execute(self) -> Self::Output {
        // Simulate some computation
        std::thread::sleep(std::time::Duration::from_millis(10));
        self.input * self.input
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Instant;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.active_count(), 4);
        pool.shutdown();
    }

    #[test]
    fn test_task_execution() {
        let pool = ThreadPool::new(2);
        
        let task = ComputeTask { input: 5 };
        let handle = pool.submit(task);
        let result = handle.get();
        
        assert_eq!(result, 25);
        pool.shutdown();
    }

    #[test]
    fn test_multiple_tasks() {
        let pool = ThreadPool::new(3);
        let mut handles = Vec::new();
        
        for i in 1..=10 {
            let task = ComputeTask { input: i };
            handles.push(pool.submit(task));
        }
        
        let results: Vec<i32> = handles.into_iter().map(|h| h.get()).collect();
        let expected: Vec<i32> = (1..=10).map(|x| x * x).collect();
        
        // Results might be in different order due to parallel execution
        let mut sorted_results = results;
        sorted_results.sort();
        assert_eq!(sorted_results, expected);
        
        pool.shutdown();
    }

    #[test]
    fn test_try_get() {
        let pool = ThreadPool::new(1);
        
        // Submit a task that takes some time
        let task = ComputeTask { input: 10 };
        let handle = pool.submit(task);
        
        // Should not be ready immediately
        assert!(handle.try_get().is_none());
        
        // Wait a bit and try again
        thread::sleep(std::time::Duration::from_millis(20));
        assert_eq!(handle.try_get(), Some(100));
        
        pool.shutdown();
    }

    #[test]
    fn test_performance() {
        // Test that parallel execution is actually faster
        let pool = ThreadPool::new(4);
        let task_count = 20;
        
        let start = Instant::now();
        let mut handles = Vec::new();
        
        for i in 1..=task_count {
            let task = ComputeTask { input: i };
            handles.push(pool.submit(task));
        }
        
        let results: Vec<i32> = handles.into_iter().map(|h| h.get()).collect();
        let parallel_time = start.elapsed();
        
        // Sequential execution for comparison
        let start = Instant::now();
        let sequential_results: Vec<i32> = (1..=task_count)
            .map(|i| {
                thread::sleep(std::time::Duration::from_millis(10));
                i * i
            })
            .collect();
        let sequential_time = start.elapsed();
        
        // Parallel should be significantly faster
        assert!(parallel_time < sequential_time / 2);
        
        // Same results
        let mut sorted_results = results;
        sorted_results.sort();
        assert_eq!(sorted_results, sequential_results);
        
        pool.shutdown();
    }
}