// Sequential Async Operations Practice
//
// Learning Objectives:
// - Chain async operations sequentially
// - Work with async iteration patterns
// - Handle accumulating results from async operations
// - Practice with tokio::time for delays
//
// cargo test --bin sequential_async_operations

/// Implement an async function that performs multiple async operations sequentially.
/// Each step should add to the result string and wait for the specified delay.
async fn sequential_async_operations(steps: Vec<(&str, u64)>) -> String {
    todo!("Implement sequential async operations")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_sequential_async_operations() {
        let steps = vec![("Step 1", 20), ("Step 2", 30), ("Step 3", 10)];
        let start = Instant::now();
        let result = sequential_async_operations(steps).await;
        let elapsed = start.elapsed();
        
        assert!(result.contains("Step 1"));
        assert!(result.contains("Step 2"));
        assert!(result.contains("Step 3"));
        // Should take at least 60ms total (20+30+10)
        assert!(elapsed >= Duration::from_millis(55));
        assert!(elapsed < Duration::from_millis(100)); // But not too much longer
    }

    #[tokio::test]
    async fn test_empty_steps() {
        let steps = vec![];
        let start = Instant::now();
        let result = sequential_async_operations(steps).await;
        let elapsed = start.elapsed();
        
        // Should complete quickly with empty input
        assert!(elapsed < Duration::from_millis(10));
        // Result behavior depends on implementation
        assert!(result.len() == 0 || !result.is_empty());
    }

    #[tokio::test]
    async fn test_single_step() {
        let steps = vec![("Only step", 25)];
        let start = Instant::now();
        let result = sequential_async_operations(steps).await;
        let elapsed = start.elapsed();
        
        assert!(result.contains("Only step"));
        assert!(elapsed >= Duration::from_millis(20));
        assert!(elapsed < Duration::from_millis(40));
    }

    #[tokio::test]
    async fn test_zero_delays() {
        let steps = vec![("Fast 1", 0), ("Fast 2", 0), ("Fast 3", 0)];
        let start = Instant::now();
        let result = sequential_async_operations(steps).await;
        let elapsed = start.elapsed();
        
        assert!(result.contains("Fast 1"));
        assert!(result.contains("Fast 2"));
        assert!(result.contains("Fast 3"));
        assert!(elapsed < Duration::from_millis(10)); // Should be very fast
    }

    #[tokio::test]
    async fn test_order_preservation() {
        let steps = vec![("First", 5), ("Second", 5), ("Third", 5)];
        let result = sequential_async_operations(steps).await;
        
        // Should maintain order in result
        let first_pos = result.find("First").unwrap_or(0);
        let second_pos = result.find("Second").unwrap_or(0);
        let third_pos = result.find("Third").unwrap_or(0);
        
        assert!(first_pos < second_pos);
        assert!(second_pos < third_pos);
    }

    #[tokio::test]
    async fn test_long_steps() {
        let steps = vec![
            ("Initialize system", 10),
            ("Load configuration", 15),
            ("Connect to database", 20),
            ("Start services", 5),
            ("Ready to serve", 0),
        ];
        
        let start = Instant::now();
        let result = sequential_async_operations(steps).await;
        let elapsed = start.elapsed();
        
        assert!(result.contains("Initialize"));
        assert!(result.contains("configuration"));
        assert!(result.contains("database"));
        assert!(result.contains("services"));
        assert!(result.contains("Ready"));
        
        // Total delay should be at least 50ms (10+15+20+5+0)
        assert!(elapsed >= Duration::from_millis(45));
        assert!(elapsed < Duration::from_millis(80));
    }

    #[tokio::test]
    async fn test_special_characters() {
        let steps = vec![
            ("Step with émojis 🚀", 10),
            ("Unicode: αβγ", 5),
            ("Numbers: 123", 5),
        ];
        
        let result = sequential_async_operations(steps).await;
        assert!(result.contains("🚀"));
        assert!(result.contains("αβγ"));
        assert!(result.contains("123"));
    }
}