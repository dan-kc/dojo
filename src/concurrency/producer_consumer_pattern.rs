// Producer-Consumer Pattern with Channels Practice
//
// Learning Objectives:
// - Use std::sync::mpsc for thread communication
// - Understand sender/receiver patterns
// - Handle multiple producers with single consumer
//
// cargo test --bin producer_consumer_pattern

/// Create a producer-consumer pattern where multiple producers send work items
/// to a single consumer. Each producer sends numbers from their range,
/// and the consumer collects all numbers and returns them sorted.
fn producer_consumer_pattern(producers: Vec<(i32, i32)>) -> Vec<i32> {
    todo!("Implement producer-consumer pattern with channels")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producer_consumer_pattern() {
        let producers = vec![(1, 3), (10, 12), (20, 21)]; // ranges: 1-3, 10-12, 20-21
        let mut result = producer_consumer_pattern(producers);
        result.sort();
        assert_eq!(result, vec![1, 2, 3, 10, 11, 12, 20, 21]);
        
        // Test with empty producers
        let result = producer_consumer_pattern(vec![]);
        assert_eq!(result, Vec::<i32>::new());
    }
}