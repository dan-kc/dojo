// Parallel Sum Calculation Practice
//
// Learning Objectives:
// - Split work across threads for parallel computation
// - Use shared accumulator with proper synchronization
// - Implement divide-and-conquer parallelism
//
// cargo test --bin parallel_sum

/// Implement a parallel sum calculation that splits work across threads
/// while avoiding data races. Each thread should sum a portion of the array
/// and contribute to a shared total.
fn parallel_sum(numbers: Vec<i64>, num_threads: usize) -> i64 {
    todo!("Implement parallel sum with shared accumulator")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_sum() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = parallel_sum(numbers.clone(), 3);
        let expected: i64 = numbers.iter().sum();
        assert_eq!(result, expected);
        
        let large_numbers: Vec<i64> = (1..=1000).collect();
        let result = parallel_sum(large_numbers.clone(), 4);
        let expected: i64 = large_numbers.iter().sum();
        assert_eq!(result, expected);
        
        // Test with single thread
        let result = parallel_sum(vec![1, 2, 3], 1);
        assert_eq!(result, 6);
    }
}