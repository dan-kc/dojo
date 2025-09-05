// Concurrent Counter with Arc<Mutex<T>> Practice
//
// Learning Objectives:
// - Use Arc for shared ownership across threads
// - Use Mutex for thread-safe mutable access
// - Understand the Arc<Mutex<T>> pattern
//
// cargo test --bin concurrent_counter

/// Implement a thread-safe counter that multiple threads can increment
/// concurrently. Return the final counter value after all threads complete.
fn concurrent_counter(num_threads: usize, increments_per_thread: usize) -> usize {
    todo!("Implement concurrent counter with Arc<Mutex<T>>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_counter() {
        let result = concurrent_counter(5, 100);
        assert_eq!(result, 500);
        
        let result = concurrent_counter(1, 1000);
        assert_eq!(result, 1000);
        
        let result = concurrent_counter(10, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_no_data_races() {
        // This test verifies that our implementations don't have data races
        // by running operations many times with multiple threads
        let counter_result = concurrent_counter(20, 50);
        assert_eq!(counter_result, 1000);
        
        // Run multiple times to catch potential race conditions
        for _ in 0..10 {
            let result = concurrent_counter(10, 10);
            assert_eq!(result, 100);
        }
    }
}