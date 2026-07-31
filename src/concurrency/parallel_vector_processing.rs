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
    fn test_parallel_vector_processing() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let result = parallel_vector_processing(input, 2);
        assert_eq!(result, vec![2, 4, 6, 8, 10, 12]);
        
        let input = vec![10, 20, 30];
        let result = parallel_vector_processing(input, 3);
        assert_eq!(result, vec![20, 40, 60]);
        
        let empty: Vec<i32> = vec![];
        let result = parallel_vector_processing(empty, 2);
        assert_eq!(result, Vec::<i32>::new());
    }
}
