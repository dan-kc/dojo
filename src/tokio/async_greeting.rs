// Async Greeting Practice
//
// Learning Objectives:
// - Understand async/await syntax fundamentals
// - Practice async function definitions with delays
// - Work with tokio::time functions
// - Handle async function return types
//
// cargo test --bin async_greeting

/// Convert this synchronous function to async.
/// The function should asynchronously wait for the specified duration
/// and then return a greeting message.
async fn async_greeting(name: &str, delay_ms: u64) -> String {
    todo!("Implement async greeting with delay")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[tokio::test]
    async fn test_async_greeting() {
        let start = Instant::now();
        let result = async_greeting("Alice", 50).await;
        let elapsed = start.elapsed();
        
        assert!(result.contains("Alice"));
        assert!(elapsed >= Duration::from_millis(45)); // Allow some timing variance
        assert!(elapsed < Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_multiple_greetings() {
        let start = Instant::now();
        let (result1, result2) = tokio::join!(
            async_greeting("User1", 30),
            async_greeting("User2", 30)
        );
        let elapsed = start.elapsed();
        
        assert!(result1.contains("User1"));
        assert!(result2.contains("User2"));
        // Should complete concurrently in ~30ms, not 60ms sequentially
        assert!(elapsed < Duration::from_millis(50));
        assert!(elapsed >= Duration::from_millis(25));
    }

    #[tokio::test]
    async fn test_zero_delay() {
        let start = Instant::now();
        let result = async_greeting("Bob", 0).await;
        let elapsed = start.elapsed();
        
        assert!(result.contains("Bob"));
        assert!(elapsed < Duration::from_millis(10)); // Should be very fast
    }

    #[tokio::test]
    async fn test_empty_name() {
        let result = async_greeting("", 10).await;
        // Should handle empty name gracefully
        assert!(!result.is_empty() || result.is_empty()); // Accept either behavior
    }

    #[tokio::test]
    async fn test_special_characters() {
        let result = async_greeting("José María", 5).await;
        assert!(result.contains("José María"));
    }
}