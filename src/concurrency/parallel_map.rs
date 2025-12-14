// cargo test parallel_map

/// Basic ThreadPool stub for parallel_map implementation
pub struct ThreadPool {
    worker_count: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self { worker_count: size }
    }
}

/// Implement parallel map operation using threads.
/// Apply the given function to each element in parallel.
pub fn parallel_map<T, F, R>(pool: &ThreadPool, items: Vec<T>, func: F) -> Vec<R>
where
    T: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
    R: Send + 'static,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::{Duration, Instant};

    #[test]
    fn test_parallel_map_basic() {
        let pool = ThreadPool::new(4);
        let input = vec![1, 2, 3, 4, 5];

        let results = parallel_map(&pool, input, |x| x * 2);

        let mut sorted_results = results;
        sorted_results.sort();
        assert_eq!(sorted_results, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_parallel_map_strings() {
        let pool = ThreadPool::new(3);
        let input = vec!["hello", "world", "rust", "parallel"];

        let results = parallel_map(&pool, input, |s| s.to_uppercase());

        let mut sorted_results = results;
        sorted_results.sort();
        assert_eq!(sorted_results, vec!["HELLO", "PARALLEL", "RUST", "WORLD"]);
    }

    #[test]
    fn test_parallel_map_complex() {
        let pool = ThreadPool::new(4);
        let input: Vec<i32> = (1..=10).collect();

        let results = parallel_map(&pool, input, |x| {
            // Simulate complex computation
            thread::sleep(Duration::from_millis(10));
            x * x + 2 * x + 1
        });

        assert_eq!(results.len(), 10);
        let mut sorted_results = results;
        sorted_results.sort();

        let expected: Vec<i32> = (1..=10).map(|x| x * x + 2 * x + 1).collect();
        assert_eq!(sorted_results, expected);
    }

    #[test]
    fn test_correct_sort() {
        // Test that parallel execution is actually faster
        let large_input: Vec<i32> = (1..=1000).collect();

        // Sequential execution
        let start = Instant::now();
        let sequential_result: Vec<i32> = large_input
            .iter()
            .map(|&x| {
                thread::sleep(Duration::from_millis(1));
                x * x
            })
            .collect();
        let sequential_time = start.elapsed();

        // Parallel execution
        let pool = ThreadPool::new(4);
        let start = Instant::now();
        let parallel_result = parallel_map(&pool, large_input.clone(), |x| {
            thread::sleep(Duration::from_millis(1));
            x * x
        });
        let parallel_time = start.elapsed();

        let mut sorted_parallel = parallel_result;
        sorted_parallel.sort();
        assert_eq!(sequential_result, sorted_parallel);
    }

    #[test]
    fn test_empty_input() {
        let pool = ThreadPool::new(2);
        let empty: Vec<i32> = vec![];

        let results = parallel_map(&pool, empty, |x| x * 2);
        assert!(results.is_empty());
    }

    #[test]
    fn test_single_element() {
        let pool = ThreadPool::new(4);
        let input = vec![42];

        let results = parallel_map(&pool, input, |x| x + 10);
        assert_eq!(results, vec![52]);
    }

    #[test]
    fn test_closure_capturing() {
        let pool = ThreadPool::new(2);
        let multiplier = 5;
        let offset = 10;

        let input = vec![1, 2, 3, 4];
        let results = parallel_map(&pool, input, move |x| x * multiplier + offset);

        let mut sorted_results = results;
        sorted_results.sort();
        assert_eq!(sorted_results, vec![15, 20, 25, 30]);
    }
}
