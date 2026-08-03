// cargo test producer_consumer_pattern

/// Create a producer-consumer pattern where multiple producers send work items
/// to a single consumer. Each producer sends numbers from their range,
/// and the consumer collects all numbers and returns them sorted.
fn producer_consumer_pattern(producers: Vec<(i32, i32)>) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_values_from_multiple_producers_in_sorted_order() {
        let producers = vec![(20, 21), (1, 3), (10, 12)];
        let result = producer_consumer_pattern(producers);

        assert_eq!(result, vec![1, 2, 3, 10, 11, 12, 20, 21]);
    }

    #[test]
    fn returns_an_empty_vec_when_there_are_no_producers() {
        let result = producer_consumer_pattern(vec![]);

        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn collects_a_single_value_range() {
        let result = producer_consumer_pattern(vec![(5, 5)]);

        assert_eq!(result, vec![5]);
    }

    #[test]
    fn collects_negative_and_zero_values() {
        let result = producer_consumer_pattern(vec![(-3, 0)]);

        assert_eq!(result, vec![-3, -2, -1, 0]);
    }

    #[test]
    fn preserves_duplicates_from_overlapping_ranges() {
        let result = producer_consumer_pattern(vec![(1, 3), (2, 4)]);

        assert_eq!(result, vec![1, 2, 2, 3, 3, 4]);
    }

    #[test]
    fn ignores_a_range_whose_start_is_greater_than_its_end() {
        let result = producer_consumer_pattern(vec![(3, 1)]);

        assert_eq!(result, Vec::<i32>::new());
    }
}
