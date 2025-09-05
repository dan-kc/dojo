// Cancellable Task Practice
//
// Learning Objectives:
// - Implement task cancellation mechanisms
// - Use tokio::select! for cancellation
// - Handle graceful task termination
// - Work with timeout and cancellation patterns
//
// cargo test --bin cancellable_task

/// Implement a task that can be cancelled gracefully.
/// Use a cancellation token or similar mechanism.
async fn cancellable_task(
    work_duration_ms: u64,
    cancel_after_ms: Option<u64>,
) -> Result<String, &'static str> {
    todo!("Implement cancellable task")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_successful_completion() {
        let result = cancellable_task(30, None).await;
        assert!(result.is_ok());
        let result_value = result.unwrap();
        assert!(result_value.contains("completed") || !result_value.is_empty());
    }

    #[tokio::test]
    async fn test_cancellation() {
        let start = Instant::now();
        let result = cancellable_task(100, Some(50)).await;
        let elapsed = start.elapsed();
        
        assert!(result.is_err());
        assert!(elapsed < Duration::from_millis(80)); // Cancelled before completion
        assert!(elapsed >= Duration::from_millis(40)); // But after cancel time
    }

    #[tokio::test]
    async fn test_no_cancellation_needed() {
        // Task completes before cancellation time
        let start = Instant::now();
        let result = cancellable_task(20, Some(50)).await;
        let elapsed = start.elapsed();
        
        assert!(result.is_ok());
        assert!(elapsed < Duration::from_millis(40)); // Completed early
        assert!(elapsed >= Duration::from_millis(15)); // But took some time
    }

    #[tokio::test]
    async fn test_immediate_cancellation() {
        let start = Instant::now();
        let result = cancellable_task(100, Some(0)).await;
        let elapsed = start.elapsed();
        
        // Should be cancelled almost immediately
        assert!(result.is_err());
        assert!(elapsed < Duration::from_millis(20));
    }

    #[tokio::test]
    async fn test_zero_work_duration() {
        let result = cancellable_task(0, None).await;
        assert!(result.is_ok());
        
        let result_with_cancel = cancellable_task(0, Some(10)).await;
        assert!(result_with_cancel.is_ok()); // Should complete before cancellation
    }

    #[tokio::test]
    async fn test_timing_precision() {
        // Test with precise timing
        let work_time = 60;
        let cancel_time = 40;
        
        let start = Instant::now();
        let result = cancellable_task(work_time, Some(cancel_time)).await;
        let elapsed = start.elapsed();
        
        assert!(result.is_err());
        // Should be cancelled around the cancel time
        assert!(elapsed >= Duration::from_millis(cancel_time - 10));
        assert!(elapsed < Duration::from_millis(cancel_time + 20));
    }

    #[tokio::test]
    async fn test_long_running_task() {
        let start = Instant::now();
        let result = cancellable_task(500, Some(100)).await;
        let elapsed = start.elapsed();
        
        assert!(result.is_err());
        // Should be cancelled well before the full work duration
        assert!(elapsed < Duration::from_millis(200));
        assert!(elapsed >= Duration::from_millis(90));
    }

    #[tokio::test]
    async fn test_edge_case_equal_times() {
        // Work duration equals cancellation time
        let duration = 50;
        let result = cancellable_task(duration, Some(duration)).await;
        
        // Could go either way depending on implementation and timing
        // Both outcomes are acceptable
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_multiple_cancellations() {
        // Test multiple cancellable tasks running concurrently
        let results = tokio::join!(
            cancellable_task(80, Some(30)),
            cancellable_task(40, None),
            cancellable_task(100, Some(60))
        );
        
        assert!(results.0.is_err()); // Should be cancelled
        assert!(results.1.is_ok());  // Should complete
        assert!(results.2.is_err()); // Should be cancelled
    }

    #[tokio::test]
    async fn test_cancellation_cleanup() {
        // Test that cancelled tasks clean up properly
        let mut successful_cancellations = 0;
        
        for _ in 0..5 {
            let result = cancellable_task(100, Some(25)).await;
            if result.is_err() {
                successful_cancellations += 1;
            }
        }
        
        // Most should be cancelled
        assert!(successful_cancellations >= 4);
    }
}