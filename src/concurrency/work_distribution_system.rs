// Work Distribution System with Bidirectional Channels Practice
//
// Learning Objectives:
// - Implement bidirectional communication between threads
// - Distribute work items to multiple workers
// - Collect results from worker threads
//
// cargo test --bin work_distribution_system

/// Implement a work distribution system where a main thread distributes work
/// to multiple worker threads via channels. Each worker processes the work
/// and sends results back. Return the sum of all processed results.
/// Work items are numbers, and processing means squaring the number.
fn work_distribution_system(work_items: Vec<i32>, num_workers: usize) -> i64 {
    todo!("Implement work distribution with bidirectional channels")
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