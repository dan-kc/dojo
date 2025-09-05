# Task Spawning and Result Collection

## Solution

```rust
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};

async fn spawn_and_sum_tasks(task_count: usize, computation_delay_ms: u64) -> i32 {
    if task_count == 0 {
        return 0;
    }
    
    let mut handles: Vec<JoinHandle<i32>> = Vec::with_capacity(task_count);
    
    // Spawn all tasks
    for i in 0..task_count {
        let handle = tokio::spawn(async move {
            // Simulate async computation
            sleep(Duration::from_millis(computation_delay_ms)).await;
            
            // Return some computed value (example: task index + 1)
            (i + 1) as i32
        });
        
        handles.push(handle);
    }
    
    // Collect all results
    let mut sum = 0;
    for handle in handles {
        match handle.await {
            Ok(result) => sum += result,
            Err(_) => {
                // Handle join error (task panicked)
                eprintln!("Task failed");
            }
        }
    }
    
    sum
}
```

## Alternative Implementation with try_join_all

```rust
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use futures::future::try_join_all;

async fn spawn_and_sum_tasks(task_count: usize, computation_delay_ms: u64) -> i32 {
    if task_count == 0 {
        return 0;
    }
    
    let handles: Vec<JoinHandle<i32>> = (0..task_count)
        .map(|i| {
            tokio::spawn(async move {
                sleep(Duration::from_millis(computation_delay_ms)).await;
                
                // More complex computation example
                let base_value = (i + 1) as i32;
                base_value * base_value - base_value + 1
            })
        })
        .collect();
    
    // Wait for all tasks and sum results
    match try_join_all(handles).await {
        Ok(results) => results.iter().sum(),
        Err(_) => 0, // Return 0 if any task failed
    }
}
```

## With Error Handling and Complex Computation

```rust
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

async fn spawn_and_sum_tasks_advanced(
    task_count: usize, 
    computation_delay_ms: u64
) -> Result<i32, String> {
    if task_count == 0 {
        return Ok(0);
    }
    
    let task_counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(task_count);
    
    for i in 0..task_count {
        let counter = Arc::clone(&task_counter);
        let handle = tokio::spawn(async move {
            // Increment counter to track task execution
            counter.fetch_add(1, Ordering::SeqCst);
            
            // Simulate work
            sleep(Duration::from_millis(computation_delay_ms)).await;
            
            // Complex computation
            let result = match i % 4 {
                0 => i * 2,
                1 => i + 10,
                2 => i.pow(2),
                _ => i * 3 + 1,
            };
            
            result as i32
        });
        
        handles.push(handle);
    }
    
    let mut sum = 0;
    let mut failed_tasks = 0;
    
    for (index, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => sum += result,
            Err(e) => {
                failed_tasks += 1;
                eprintln!("Task {} failed: {}", index, e);
            }
        }
    }
    
    let completed_tasks = task_counter.load(Ordering::SeqCst);
    
    if failed_tasks > task_count / 2 {
        Err(format!("Too many tasks failed: {}/{}", failed_tasks, task_count))
    } else {
        println!("Completed {}/{} tasks successfully", completed_tasks - failed_tasks, task_count);
        Ok(sum)
    }
}
```

## Explanation

The task spawning and result collection function demonstrates core tokio concurrency patterns:

1. **Task Spawning**: `tokio::spawn` creates independent async tasks that run concurrently on the tokio runtime.

2. **JoinHandle Management**: Each spawned task returns a `JoinHandle` that can be awaited to get the task's result.

3. **Concurrent Execution**: All tasks start immediately and run concurrently, rather than sequentially.

4. **Result Collection**: The function waits for all tasks to complete and accumulates their results.

5. **Error Handling**: `JoinHandle::await` returns a `Result` that must be handled, as tasks can panic or be cancelled.

6. **Performance Benefits**: Concurrent execution means the total time is approximately the time of the longest task, not the sum of all task times.

7. **Resource Considerations**: Each spawned task has some overhead, so this pattern is best for I/O-bound or CPU-intensive work that benefits from parallelism.

This pattern is fundamental to tokio-based applications and demonstrates how to coordinate multiple independent async operations while collecting their results.