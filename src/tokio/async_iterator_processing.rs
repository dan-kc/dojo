// Async Iterator Processing Practice
//
// Learning Objectives:
// - Convert synchronous iteration to async patterns
// - Process collections with async operations
// - Handle delays between async operations
// - Work with async loops and iteration
//
// cargo test --bin async_iterator_processing

/// Convert a synchronous iterator operation to async.
/// Process each item asynchronously with a delay between items.
async fn async_iterator_processing(items: Vec<i32>, delay_per_item_ms: u64) -> Vec<i32> {
    todo!("Implement async iterator processing")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_async_iterator_processing() {
        let items = vec![1, 2, 3];
        let start = Instant::now();
        let result = async_iterator_processing(items, 20).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result.len(), 3);
        // Should process 3 items with 20ms delay each = ~60ms
        assert!(elapsed >= Duration::from_millis(50));
        assert!(elapsed < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_empty_collection() {
        let items = vec![];
        let start = Instant::now();
        let result = async_iterator_processing(items, 50).await;
        let elapsed = start.elapsed();
        
        assert!(result.is_empty());
        assert!(elapsed < Duration::from_millis(10)); // Should be fast for empty
    }

    #[tokio::test]
    async fn test_single_item() {
        let items = vec![42];
        let start = Instant::now();
        let result = async_iterator_processing(items, 30).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result.len(), 1);
        assert!(result.contains(&42) || result[0] != 42); // Allow transformation
        assert!(elapsed >= Duration::from_millis(25));
        assert!(elapsed < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_zero_delay() {
        let items = vec![1, 2, 3, 4, 5];
        let start = Instant::now();
        let result = async_iterator_processing(items, 0).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result.len(), 5);
        assert!(elapsed < Duration::from_millis(10)); // Should be very fast
    }

    #[tokio::test]
    async fn test_processing_order() {
        let items = vec![10, 20, 30, 40];
        let result = async_iterator_processing(items.clone(), 5).await;
        
        // Results should maintain some relationship to input
        assert_eq!(result.len(), items.len());
        // The exact transformation depends on implementation
    }

    #[tokio::test]
    async fn test_large_collection() {
        let items: Vec<i32> = (1..=10).collect();
        let start = Instant::now();
        let result = async_iterator_processing(items, 5).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result.len(), 10);
        // Should take at least 50ms (10 items * 5ms each)
        assert!(elapsed >= Duration::from_millis(40));
        assert!(elapsed < Duration::from_millis(80));
    }

    #[tokio::test]
    async fn test_negative_numbers() {
        let items = vec![-5, -10, 15, -20];
        let result = async_iterator_processing(items, 1).await;
        
        assert_eq!(result.len(), 4);
        // Should handle negative numbers appropriately
    }

    #[tokio::test]
    async fn test_timing_precision() {
        let items = vec![1, 2];
        let delay = 100; // Longer delay for more precise timing
        
        let start = Instant::now();
        let _result = async_iterator_processing(items, delay).await;
        let elapsed = start.elapsed();
        
        // Should take at least 200ms (2 items * 100ms each)
        assert!(elapsed >= Duration::from_millis(190));
        assert!(elapsed < Duration::from_millis(250));
    }

    #[tokio::test]
    async fn test_concurrent_processing() {
        // Compare with concurrent processing of independent collections
        let items1 = vec![1, 2, 3];
        let items2 = vec![4, 5, 6];
        
        let start = Instant::now();
        let (result1, result2) = tokio::join!(
            async_iterator_processing(items1, 20),
            async_iterator_processing(items2, 20)
        );
        let elapsed = start.elapsed();
        
        assert_eq!(result1.len(), 3);
        assert_eq!(result2.len(), 3);
        
        // Both should complete in ~60ms (concurrent), not 120ms (sequential)
        assert!(elapsed < Duration::from_millis(80));
        assert!(elapsed >= Duration::from_millis(50));
    }
}