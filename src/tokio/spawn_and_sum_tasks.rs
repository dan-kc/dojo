// Task Spawning and Result Collection Practice
//
// Learning Objectives:
// - Use tokio::spawn for task creation
// - Handle JoinHandle in async context
// - Collect results from multiple spawned tasks
// - Practice concurrent task execution
//
// cargo test --bin spawn_and_sum_tasks

/// Spawn multiple async tasks that each perform a computation.
/// Wait for all tasks to complete and return the sum of their results.
async fn spawn_and_sum_tasks(task_count: usize, computation_delay_ms: u64) -> i32 {
    todo!("Implement task spawning and result collection")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_spawn_and_sum_tasks() {
        let start = Instant::now();
        let result = spawn_and_sum_tasks(5, 50).await;
        let elapsed = start.elapsed();
        
        // Should sum task results (depends on implementation)
        assert!(result > 0);
        // Should complete in ~50ms (concurrent), not 250ms (sequential)
        assert!(elapsed < Duration::from_millis(100));
        assert!(elapsed >= Duration::from_millis(45));
    }

    #[tokio::test]
    async fn test_single_task() {
        let start = Instant::now();
        let result = spawn_and_sum_tasks(1, 30).await;
        let elapsed = start.elapsed();
        
        assert!(result > 0);
        assert!(elapsed >= Duration::from_millis(25));
        assert!(elapsed < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_zero_tasks() {
        let start = Instant::now();
        let result = spawn_and_sum_tasks(0, 100).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result, 0);
        assert!(elapsed < Duration::from_millis(10)); // Should be very fast
    }

    #[tokio::test]
    async fn test_many_tasks() {
        let start = Instant::now();
        let result = spawn_and_sum_tasks(10, 25).await;
        let elapsed = start.elapsed();
        
        assert!(result > 0);
        // With 10 concurrent tasks, should still complete in ~25ms
        assert!(elapsed < Duration::from_millis(50));
        assert!(elapsed >= Duration::from_millis(20));
    }

    #[tokio::test]
    async fn test_zero_delay() {
        let start = Instant::now();
        let result = spawn_and_sum_tasks(8, 0).await;
        let elapsed = start.elapsed();
        
        assert!(result > 0);
        assert!(elapsed < Duration::from_millis(20)); // Should be very fast
    }

    #[tokio::test]
    async fn test_concurrent_performance() {
        // Verify that spawning tasks actually provides concurrency benefits
        let task_count = 8;
        let delay_per_task = 30;
        
        let start = Instant::now();
        let _result = spawn_and_sum_tasks(task_count, delay_per_task).await;
        let elapsed = start.elapsed();
        
        // Should be much faster than sequential execution
        let sequential_time = Duration::from_millis(task_count as u64 * delay_per_task);
        assert!(elapsed < sequential_time / 2);
    }

    #[tokio::test]
    async fn test_result_accumulation() {
        // Test with different task counts to verify proper accumulation
        let result_3 = spawn_and_sum_tasks(3, 10).await;
        let result_6 = spawn_and_sum_tasks(6, 10).await;
        
        // More tasks should generally produce larger results
        // (exact relationship depends on implementation)
        assert!(result_3 > 0);
        assert!(result_6 > 0);
        assert!(result_6 > result_3 || result_6 == result_3); // Allow equal for constant computations
    }

    #[tokio::test]
    async fn test_task_independence() {
        // Tasks should be independent - running same config twice should give same result
        let result1 = spawn_and_sum_tasks(4, 15).await;
        let result2 = spawn_and_sum_tasks(4, 15).await;
        
        // Results should be consistent (assuming deterministic computation)
        assert_eq!(result1, result2);
    }

    #[tokio::test]
    async fn test_large_task_count() {
        // Test with many tasks to verify scalability
        let start = Instant::now();
        let result = spawn_and_sum_tasks(50, 5).await;
        let elapsed = start.elapsed();
        
        assert!(result > 0);
        // Should still complete relatively quickly due to concurrency
        assert!(elapsed < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_deterministic_results() {
        // Multiple runs with same parameters should produce same results
        let params = (5, 20);
        
        let result1 = spawn_and_sum_tasks(params.0, params.1).await;
        let result2 = spawn_and_sum_tasks(params.0, params.1).await;
        let result3 = spawn_and_sum_tasks(params.0, params.1).await;
        
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
}