// cargo test --lib concurrency::work_distribution_system::

//! Use num_workers worker threads and channels to square each item exactly once and return the sum. Join all workers. Zero workers must panic; empty input with positive workers returns zero. Convert operands to i64 before multiplication; assume the final sum fits i64.

/// Implement a work distribution system where a main thread distributes work
/// to multiple worker threads via channels. Each worker processes the work
/// and sends results back. Return the sum of all processed results.
/// Work items are numbers, and processing means squaring the number.
fn work_distribution_system(work_items: Vec<i32>, num_workers: usize) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_distribution_system() {
        let work = vec![1, 2, 3, 4, 5];
        let result = work_distribution_system(work, 2);
        // 1^2 + 2^2 + 3^2 + 4^2 + 5^2 = 1 + 4 + 9 + 16 + 25 = 55
        assert_eq!(result, 55);

        let work = vec![10];
        let result = work_distribution_system(work, 1);
        assert_eq!(result, 100); // 10^2 = 100

        // Test with more workers than work items
        let work = vec![2, 3];
        let result = work_distribution_system(work, 5);
        assert_eq!(result, 13); // 2^2 + 3^2 = 4 + 9 = 13
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test]
    #[should_panic]
    fn rejects_zero_workers() {
        work_distribution_system(vec![], 0);
    }
    #[test]
    fn converts_before_squaring() {
        assert_eq!(
            work_distribution_system(vec![i32::MIN], 2),
            4611686018427387904
        );
    }
}
