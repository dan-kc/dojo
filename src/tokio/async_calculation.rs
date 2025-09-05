// Async Calculation Practice
//
// Learning Objectives:
// - Create async functions that perform computations with delays
// - Understand async function return types
// - Practice combining async operations with computation
// - Work with tokio::time for simulated async work
//
// cargo test async_calculation

/// Create an async function that simulates an async computation.
/// It should perform the calculation after an async delay.
async fn async_calculation(a: i32, b: i32, delay_ms: u64) -> i32 {
    todo!("Implement async calculation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_async_calculation() {
        let start = Instant::now();
        let result = async_calculation(5, 10, 30).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result, 15);
        assert!(elapsed >= Duration::from_millis(25));
        assert!(elapsed < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_calculation_accuracy() {
        let result1 = async_calculation(100, 200, 5).await;
        assert_eq!(result1, 300);

        let result2 = async_calculation(-10, 25, 5).await;
        assert_eq!(result2, 15);

        let result3 = async_calculation(0, 42, 5).await;
        assert_eq!(result3, 42);
    }

    #[tokio::test]
    async fn test_zero_delay() {
        let start = Instant::now();
        let result = async_calculation(7, 8, 0).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result, 15);
        assert!(elapsed < Duration::from_millis(5)); // Should be very fast
    }

    #[tokio::test]
    async fn test_multiple_calculations() {
        let start = Instant::now();
        let (result1, result2, result3) = tokio::join!(
            async_calculation(1, 2, 20),
            async_calculation(3, 4, 20),
            async_calculation(5, 6, 20)
        );
        let elapsed = start.elapsed();
        
        assert_eq!(result1, 3);
        assert_eq!(result2, 7);
        assert_eq!(result3, 11);
        
        // Should complete concurrently in ~20ms, not 60ms sequentially
        assert!(elapsed < Duration::from_millis(40));
        assert!(elapsed >= Duration::from_millis(15));
    }

    #[tokio::test]
    async fn test_large_numbers() {
        let result = async_calculation(1_000_000, 2_000_000, 1).await;
        assert_eq!(result, 3_000_000);
    }

    #[tokio::test]
    async fn test_negative_numbers() {
        let result1 = async_calculation(-5, -10, 5).await;
        assert_eq!(result1, -15);

        let result2 = async_calculation(-100, 50, 5).await;
        assert_eq!(result2, -50);
    }

    #[tokio::test]
    async fn test_sequential_vs_concurrent() {
        // Sequential execution
        let start = Instant::now();
        let seq1 = async_calculation(1, 1, 25).await;
        let seq2 = async_calculation(2, 2, 25).await;
        let seq3 = async_calculation(3, 3, 25).await;
        let sequential_time = start.elapsed();
        
        // Concurrent execution
        let start = Instant::now();
        let (conc1, conc2, conc3) = tokio::join!(
            async_calculation(1, 1, 25),
            async_calculation(2, 2, 25),
            async_calculation(3, 3, 25)
        );
        let concurrent_time = start.elapsed();
        
        // Results should be the same
        assert_eq!(seq1, conc1);
        assert_eq!(seq2, conc2);
        assert_eq!(seq3, conc3);
        
        // Concurrent should be significantly faster
        assert!(concurrent_time < sequential_time / 2);
        assert!(sequential_time >= Duration::from_millis(70)); // ~75ms
        assert!(concurrent_time < Duration::from_millis(40));  // ~25ms
    }
}