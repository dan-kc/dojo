// Parallel Reduce Operation Practice
//
// Learning Objectives:
// - Implement parallel reduction using divide-and-conquer
// - Use threads for parallel computation
// - Handle associative operations in parallel
// - Combine partial results efficiently
//
// cargo test --bin parallel_reduce

/// Basic ThreadPool stub for parallel_reduce implementation
pub struct ThreadPool {
    worker_count: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self { worker_count: size }
    }
}

/// Implement parallel reduce operation.
/// Apply the reduction function in parallel using a divide-and-conquer approach.
pub fn parallel_reduce<T, F>(
    pool: &ThreadPool,
    items: Vec<T>,
    identity: T,
    reduce_fn: F,
) -> T
where
    T: Send + Clone + 'static,
    F: Fn(T, T) -> T + Send + Sync + Copy + 'static,
{
    todo!("Implement parallel_reduce using thread pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_reduce_sum() {
        let pool = ThreadPool::new(4);
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        let sum = parallel_reduce(&pool, input.clone(), 0, |a, b| a + b);
        assert_eq!(sum, 55); // 1+2+...+10 = 55
    }

    #[test]
    fn test_parallel_reduce_product() {
        let pool = ThreadPool::new(4);
        let input = vec![1, 2, 3, 4];
        
        let product = parallel_reduce(&pool, input, 1, |a, b| a * b);
        assert_eq!(product, 24); // 1*2*3*4 = 24
    }

    #[test]
    fn test_parallel_reduce_max() {
        let pool = ThreadPool::new(3);
        let input = vec![5, 2, 9, 1, 8, 3];
        
        let max = parallel_reduce(&pool, input, i32::MIN, |a, b| a.max(b));
        assert_eq!(max, 9);
    }

    #[test]
    fn test_parallel_reduce_min() {
        let pool = ThreadPool::new(2);
        let input = vec![5, 2, 9, 1, 8, 3];
        
        let min = parallel_reduce(&pool, input, i32::MAX, |a, b| a.min(b));
        assert_eq!(min, 1);
    }

    #[test]
    fn test_empty_collection() {
        let pool = ThreadPool::new(4);
        let empty: Vec<i32> = vec![];
        
        let result = parallel_reduce(&pool, empty, 42, |a, b| a + b);
        assert_eq!(result, 42); // Should return identity value
    }

    #[test]
    fn test_single_element() {
        let pool = ThreadPool::new(2);
        let input = vec![100];
        
        let result = parallel_reduce(&pool, input, 0, |a, b| a + b);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_string_concatenation() {
        let pool = ThreadPool::new(3);
        let input = vec!["Hello", " ", "World", "!"];
        
        let result = parallel_reduce(&pool, input, "", |a, b| {
            format!("{}{}", a, b)
        });
        
        // Note: parallel reduction might not preserve exact order
        // but should contain all elements
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
        assert!(result.contains("!"));
    }

    #[test]
    fn test_large_dataset() {
        let pool = ThreadPool::new(8);
        let input: Vec<i64> = (1..=1000).collect();
        
        let sum = parallel_reduce(&pool, input.clone(), 0i64, |a, b| a + b);
        let expected_sum = (1000 * 1001) / 2; // Sum formula: n(n+1)/2
        
        assert_eq!(sum, expected_sum);
    }

    #[test]
    fn test_associative_property() {
        let pool = ThreadPool::new(4);
        let input = vec![2, 3, 4, 5];
        
        // Test with multiplication (associative)
        let result1 = parallel_reduce(&pool, input.clone(), 1, |a, b| a * b);
        let expected = 2 * 3 * 4 * 5;
        assert_eq!(result1, expected);
        
        // Test with subtraction (not associative - order matters)
        // Result may vary with parallel execution
        let result2 = parallel_reduce(&pool, input, 0, |a, b| a - b);
        // Just ensure it completes without panic
        assert!(result2.abs() <= 100);
    }

    #[test]
    fn test_boolean_operations() {
        let pool = ThreadPool::new(2);
        
        // Test AND operation
        let all_true = vec![true, true, true, true];
        let and_result = parallel_reduce(&pool, all_true, true, |a, b| a && b);
        assert_eq!(and_result, true);
        
        let mixed_bool = vec![true, false, true, true];
        let and_result2 = parallel_reduce(&pool, mixed_bool, true, |a, b| a && b);
        assert_eq!(and_result2, false);
        
        // Test OR operation
        let some_true = vec![false, false, true, false];
        let or_result = parallel_reduce(&pool, some_true, false, |a, b| a || b);
        assert_eq!(or_result, true);
    }
}