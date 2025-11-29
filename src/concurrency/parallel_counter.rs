// cargo test parallel_counter

/// Spawn multiple threads that each increment a counter and return their results.
/// Each thread should sleep for `sleep_ms` milliseconds before returning its result.
/// The function should wait for all threads to complete and return the sum of all results.
fn parallel_counter(num_threads: usize, sleep_ms: u64) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_parallel_counter() {
        let result = parallel_counter(5, 10);
        assert_eq!(result, 5); // Each thread returns 1, sum is 5

        let result = parallel_counter(0, 10);
        assert_eq!(result, 0); // No threads, sum is 0
    }

    #[test]
    fn test_thread_timing() {
        let start = std::time::Instant::now();
        parallel_counter(3, 100);
        let duration = start.elapsed();

        // Should complete in roughly 100ms, not 300ms (parallel execution)
        assert!(duration.as_millis() < 200);
    }
}
