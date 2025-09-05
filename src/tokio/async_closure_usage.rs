// Async Closure Usage Practice
//
// Learning Objectives:
// - Work with async closures and higher-order functions
// - Pass async functions as parameters
// - Use generic async functions with closure parameters
// - Handle Future trait bounds in function signatures
//
// cargo test --bin async_closure_usage

/// Convert sync closure to async and use it in async context.
async fn async_closure_usage<F, Fut>(items: Vec<i32>, async_processor: F) -> Vec<i32>
where
    F: Fn(i32) -> Fut,
    Fut: std::future::Future<Output = i32>,
{
    todo!("Implement async closure usage")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_async_closure_usage() {
        let items = vec![1, 2, 3];
        let processor = |x| async move {
            sleep(Duration::from_millis(10)).await;
            x * 2
        };
        
        let result = async_closure_usage(items, processor).await;
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_complex_processor() {
        let items = vec![5, 10, 15];
        let processor = |x| async move {
            sleep(Duration::from_millis(5)).await;
            if x % 2 == 0 {
                x / 2
            } else {
                x * 3
            }
        };
        
        let result = async_closure_usage(items, processor).await;
        assert_eq!(result, vec![15, 5, 45]); // 5*3, 10/2, 15*3
    }

    #[tokio::test]
    async fn test_empty_collection() {
        let items = vec![];
        let processor = |x| async move {
            sleep(Duration::from_millis(10)).await;
            x + 1
        };
        
        let result = async_closure_usage(items, processor).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_async_transformation() {
        let items = vec![-3, -1, 0, 1, 3];
        let processor = |x| async move {
            sleep(Duration::from_millis(1)).await;
            x.abs() + 10
        };
        
        let result = async_closure_usage(items, processor).await;
        assert_eq!(result, vec![13, 11, 10, 11, 13]);
    }

    #[tokio::test]
    async fn test_timing_behavior() {
        use std::time::Instant;
        
        let items = vec![1, 2, 3, 4];
        let processor = |x| async move {
            sleep(Duration::from_millis(25)).await;
            x * x
        };
        
        let start = Instant::now();
        let result = async_closure_usage(items, processor).await;
        let elapsed = start.elapsed();
        
        assert_eq!(result, vec![1, 4, 9, 16]);
        // Should take at least 100ms for 4 items with 25ms each
        assert!(elapsed >= Duration::from_millis(90));
        assert!(elapsed < Duration::from_millis(150));
    }

    #[tokio::test]
    async fn test_string_processing() {
        // Test with a more complex async closure that could return strings
        let items = vec![1, 2, 3];
        let processor = |x| async move {
            sleep(Duration::from_millis(5)).await;
            // Convert to "number" but return as i32 length
            format!("number_{}", x).len() as i32
        };
        
        let result = async_closure_usage(items, processor).await;
        assert_eq!(result, vec![8, 8, 8]); // "number_1", "number_2", "number_3" all have 8 chars
    }

    #[tokio::test]
    async fn test_closure_with_capture() {
        let multiplier = 5;
        let offset = 10;
        
        let items = vec![1, 2, 3];
        let processor = move |x| async move {
            sleep(Duration::from_millis(5)).await;
            x * multiplier + offset
        };
        
        let result = async_closure_usage(items, processor).await;
        assert_eq!(result, vec![15, 20, 25]); // (1*5+10), (2*5+10), (3*5+10)
    }

    #[tokio::test]
    async fn test_single_item() {
        let items = vec![42];
        let processor = |x| async move {
            sleep(Duration::from_millis(10)).await;
            x - 2
        };
        
        let result = async_closure_usage(items, processor).await;
        assert_eq!(result, vec![40]);
    }

    #[tokio::test]
    async fn test_zero_processing() {
        let items = vec![0, 0, 0];
        let processor = |x| async move {
            sleep(Duration::from_millis(1)).await;
            if x == 0 { 100 } else { x }
        };
        
        let result = async_closure_usage(items, processor).await;
        assert_eq!(result, vec![100, 100, 100]);
    }
}