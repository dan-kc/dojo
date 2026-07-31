// cargo test parallel_vector_processing

/// Spawn threads that process different parts of a vector in parallel.
/// Each thread processes a slice of the input vector by doubling each element.
/// Return a new vector with all processed elements in the correct order.
fn parallel_vector_processing(input: Vec<i32>, num_threads: usize) -> Vec<i32> {
    todo!("Implement parallel vector processing")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_elements_and_preserves_order_across_chunks() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let result = parallel_vector_processing(input, 2);

        assert_eq!(result, vec![2, 4, 6, 8, 10, 12]);
    }

    #[test]
    fn handles_one_element_per_thread() {
        let input = vec![10, 20, 30];
        let result = parallel_vector_processing(input, 3);

        assert_eq!(result, vec![20, 40, 60]);
    }

    #[test]
    fn handles_input_that_does_not_divide_evenly_between_threads() {
        let input = vec![1, 2, 3, 4, 5];
        let result = parallel_vector_processing(input, 2);

        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn handles_more_threads_than_elements() {
        let input = vec![-2, 0, 7];
        let result = parallel_vector_processing(input, 8);

        assert_eq!(result, vec![-4, 0, 14]);
    }

    #[test]
    fn handles_empty_input() {
        let result = parallel_vector_processing(Vec::new(), 2);

        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn returns_empty_output_when_no_threads_are_requested() {
        let result = parallel_vector_processing(vec![1, 2, 3], 0);

        assert_eq!(result, Vec::<i32>::new());
    }
}
