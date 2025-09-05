# Cancellable Task

## Solution

```rust
use tokio::time::{sleep, Duration};

async fn cancellable_task(
    work_duration_ms: u64,
    cancel_after_ms: Option<u64>,
) -> Result<String, &'static str> {
    match cancel_after_ms {
        Some(cancel_time) => {
            // Use tokio::select! for cancellation
            tokio::select! {
                _ = sleep(Duration::from_millis(work_duration_ms)) => {
                    Ok("Task completed successfully".to_string())
                }
                _ = sleep(Duration::from_millis(cancel_time)) => {
                    Err("Task was cancelled")
                }
            }
        }
        None => {
            // No cancellation, just complete the work
            sleep(Duration::from_millis(work_duration_ms)).await;
            Ok("Task completed without cancellation".to_string())
        }
    }
}
```

## Alternative Implementation with CancellationToken

```rust
use tokio::time::{sleep, Duration, timeout};
use tokio_util::sync::CancellationToken;

async fn cancellable_task_with_token(
    work_duration_ms: u64,
    cancel_after_ms: Option<u64>,
) -> Result<String, &'static str> {
    let token = CancellationToken::new();
    
    if let Some(cancel_time) = cancel_after_ms {
        let token_clone = token.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(cancel_time)).await;
            token_clone.cancel();
        });
    }
    
    tokio::select! {
        _ = sleep(Duration::from_millis(work_duration_ms)) => {
            Ok("Work completed".to_string())
        }
        _ = token.cancelled() => {
            Err("Task cancelled by token")
        }
    }
}
```

## With Timeout Pattern

```rust
use tokio::time::{sleep, Duration, timeout};

async fn cancellable_task_timeout(
    work_duration_ms: u64,
    cancel_after_ms: Option<u64>,
) -> Result<String, &'static str> {
    let work = async {
        sleep(Duration::from_millis(work_duration_ms)).await;
        "Task completed successfully"
    };
    
    match cancel_after_ms {
        Some(timeout_ms) => {
            match timeout(Duration::from_millis(timeout_ms), work).await {
                Ok(result) => Ok(result.to_string()),
                Err(_) => Err("Task timed out"),
            }
        }
        None => Ok(work.await.to_string()),
    }
}
```

## Advanced Implementation with Progress Tracking

```rust
use tokio::time::{sleep, Duration, Instant};

async fn cancellable_task_advanced(
    work_duration_ms: u64,
    cancel_after_ms: Option<u64>,
) -> Result<String, &'static str> {
    let start_time = Instant::now();
    let work_duration = Duration::from_millis(work_duration_ms);
    let cancel_duration = cancel_after_ms.map(Duration::from_millis);
    
    // Simulate work in small increments to allow for responsive cancellation
    let step_duration = Duration::from_millis(10);
    let total_steps = (work_duration_ms + 9) / 10; // Round up
    
    for step in 0..total_steps {
        // Check if we should cancel
        if let Some(cancel_time) = cancel_duration {
            if start_time.elapsed() >= cancel_time {
                return Err("Task cancelled");
            }
        }
        
        // Do a small amount of work
        sleep(step_duration).await;
        
        // Early completion check
        if start_time.elapsed() >= work_duration {
            let progress = ((step + 1) * 100 / total_steps).min(100);
            return Ok(format!("Task completed ({}% progress)", progress));
        }
    }
    
    Ok("Task completed fully".to_string())
}
```

## With Graceful Cleanup

```rust
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

async fn cancellable_task_with_cleanup(
    work_duration_ms: u64,
    cancel_after_ms: Option<u64>,
) -> Result<String, &'static str> {
    let cleanup_needed = Arc::new(AtomicBool::new(true));
    let cleanup_flag = cleanup_needed.clone();
    
    let work_future = async move {
        // Simulate acquiring resources
        let _resource = "acquired_resource";
        
        // Perform work
        sleep(Duration::from_millis(work_duration_ms)).await;
        
        // Mark that cleanup is no longer needed (work completed normally)
        cleanup_flag.store(false, Ordering::SeqCst);
        "Work completed successfully"
    };
    
    let result = match cancel_after_ms {
        Some(cancel_time) => {
            tokio::select! {
                result = work_future => Ok(result.to_string()),
                _ = sleep(Duration::from_millis(cancel_time)) => Err("Task cancelled"),
            }
        }
        None => Ok(work_future.await.to_string()),
    };
    
    // Perform cleanup if needed
    if cleanup_needed.load(Ordering::SeqCst) {
        // Simulate cleanup
        println!("Performing cleanup after cancellation");
        sleep(Duration::from_millis(5)).await;
    }
    
    result
}
```

## Explanation

The cancellable task demonstrates several important async patterns:

1. **tokio::select! Macro**: The primary mechanism for racing multiple async operations. The first one to complete wins.

2. **Cancellation Patterns**:
   - Timeout-based: Cancel after a specific duration
   - Token-based: Use external cancellation signals
   - Polling-based: Check cancellation status periodically

3. **Graceful Handling**: Both successful completion and cancellation are handled as valid outcomes.

4. **Resource Management**: Advanced implementations can include cleanup logic for when tasks are cancelled.

5. **Progress Tracking**: Breaking work into smaller chunks allows for more responsive cancellation.

6. **Real-world Applications**:
   - HTTP request timeouts
   - Background task cancellation
   - User-initiated operation cancellation
   - Resource cleanup on cancellation
   - Graceful shutdown patterns

The key insight is that async cancellation in Rust is cooperative - tasks must explicitly check for and handle cancellation signals, making it predictable and safe.