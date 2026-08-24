// cargo test parallel_sum

/// Implement a parallel sum calculation that splits work across threads
/// while avoiding data races. Each thread should sum a portion of the array
/// and contribute to a shared total in a mutex.
fn parallel_sum(numbers: Vec<i64>, num_threads: usize) -> i64 {
    todo!()
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

    #[test]
    fn sums_mixed_positive_and_negative_numbers() {
        let numbers = vec![-10, 4, -3, 12, -8, 5];

        assert_eq!(parallel_sum(numbers, 3), 0);
    }

    #[test]
    fn handles_more_threads_than_numbers() {
        let numbers = vec![7, -2, 11];

        assert_eq!(parallel_sum(numbers, 10), 16);
    }

    #[test]
    fn sums_a_single_number_with_multiple_threads() {
        assert_eq!(parallel_sum(vec![-42], 4), -42);
    }

    #[test]
    fn empty_input_has_a_sum_of_zero() {
        assert_eq!(parallel_sum(Vec::new(), 4), 0);
    }

    #[test]
    fn zero_threads_returns_zero() {
        assert_eq!(parallel_sum(vec![1, 2, 3], 0), 0);
    }
}
