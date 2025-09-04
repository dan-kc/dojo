// Iterator Performance Practice
//
// Learning objectives:
// - Understanding iterator performance characteristics
// - Comparing iterator vs loop approaches
// - Using iterator methods for optimal performance
// - Understanding lazy vs eager evaluation
//
// cargo test --lib iterators::iterator_performance

/// Compare different approaches for processing large datasets.
/// Implement a function that finds the sum of squares of even numbers
/// in a large vector using iterator methods for optimal performance.
pub fn sum_squares_evens_iterator(numbers: &[i32]) -> i64 {
    todo!("Use iterator chain for optimal performance")
}

/// Alternative implementation using traditional loops for comparison.
/// This is provided to help you understand the performance difference.
pub fn sum_squares_evens_loop(numbers: &[i32]) -> i64 {
    let mut sum = 0i64;
    for &n in numbers {
        if n % 2 == 0 {
            sum += (n as i64) * (n as i64);
        }
    }
    sum
}

/// Create a function that finds the first N prime numbers using iterators.
/// Use lazy evaluation to avoid computing more primes than needed.
pub fn first_n_primes(n: usize) -> Vec<u32> {
    todo!("Use iterator methods with take() for lazy evaluation")
}

/// Helper function to check if a number is prime
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    
    let sqrt_num = (num as f64).sqrt() as u32;
    for i in (3..=sqrt_num).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

/// Implement an efficient function to find the top K most frequent words
/// in a text, using iterator methods for text processing and HashMap operations.
pub fn top_k_frequent_words(text: &str, k: usize) -> Vec<String> {
    todo!("Use iterators to split, count, and find top K efficiently")
}

/// Create a function that processes a large dataset in chunks to demonstrate
/// efficient batch processing. Calculate the average of each chunk and return
/// a vector of chunk averages.
pub fn chunk_averages(numbers: &[f64], chunk_size: usize) -> Vec<f64> {
    todo!("Use chunks() iterator method for efficient batch processing")
}

/// Implement a function that demonstrates efficient parallel-like processing
/// using iterator combinators. Given two vectors of the same length,
/// compute the dot product efficiently.
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    todo!("Use zip and map with fold/sum for optimal performance")
}

/// Create a function that efficiently processes nested data structures.
/// Given a vector of vectors, find all elements that are greater than
/// the average of their respective inner vector.
pub fn elements_above_average(data: &[Vec<i32>]) -> Vec<i32> {
    todo!("Use flat_map and efficient average calculation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_squares_evens_iterator_small() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = sum_squares_evens_iterator(&numbers);
        assert_eq!(result, 220); // 4 + 16 + 36 + 64 + 100 = 220
    }

    #[test]
    fn test_sum_squares_evens_iterator_matches_loop() {
        let numbers: Vec<i32> = (1..=1000).collect();
        let iterator_result = sum_squares_evens_iterator(&numbers);
        let loop_result = sum_squares_evens_loop(&numbers);
        assert_eq!(iterator_result, loop_result);
    }

    #[test]
    fn test_sum_squares_evens_empty() {
        let numbers = [];
        let result = sum_squares_evens_iterator(&numbers);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_first_n_primes() {
        let primes = first_n_primes(5);
        assert_eq!(primes, vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_first_n_primes_zero() {
        let primes = first_n_primes(0);
        assert_eq!(primes, vec![]);
    }

    #[test]
    fn test_first_n_primes_larger() {
        let primes = first_n_primes(10);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_is_prime_helper() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(9));
        assert!(is_prime(17));
        assert!(!is_prime(15));
    }

    #[test]
    fn test_top_k_frequent_words() {
        let text = "the quick brown fox jumps over the lazy dog the fox is quick";
        let result = top_k_frequent_words(text, 3);
        // "the" appears 3 times, "quick" and "fox" appear 2 times each
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "the");
        // Note: "quick" and "fox" could be in either order since they have the same frequency
        assert!(result.contains(&"quick".to_string()));
        assert!(result.contains(&"fox".to_string()));
    }

    #[test]
    fn test_top_k_frequent_words_empty() {
        let text = "";
        let result = top_k_frequent_words(text, 3);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_top_k_frequent_words_k_larger_than_unique() {
        let text = "hello world";
        let result = top_k_frequent_words(text, 5);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"hello".to_string()));
        assert!(result.contains(&"world".to_string()));
    }

    #[test]
    fn test_chunk_averages() {
        let numbers = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = chunk_averages(&numbers, 3);
        assert_eq!(result, vec![2.0, 5.0, 7.5]); // [1,2,3]=2.0, [4,5,6]=5.0, [7,8]=7.5
    }

    #[test]
    fn test_chunk_averages_exact_chunks() {
        let numbers = [10.0, 20.0, 30.0, 40.0];
        let result = chunk_averages(&numbers, 2);
        assert_eq!(result, vec![15.0, 35.0]); // [10,20]=15.0, [30,40]=35.0
    }

    #[test]
    fn test_chunk_averages_single_element_chunks() {
        let numbers = [1.0, 2.0, 3.0];
        let result = chunk_averages(&numbers, 1);
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = dot_product(&a, &b);
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_dot_product_empty() {
        let a = [];
        let b = [];
        let result = dot_product(&a, &b);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_dot_product_single_element() {
        let a = [3.0];
        let b = [7.0];
        let result = dot_product(&a, &b);
        assert_eq!(result, 21.0);
    }

    #[test]
    fn test_elements_above_average() {
        let data = vec![
            vec![1, 2, 3, 4, 5], // avg = 3.0, above avg: 4, 5
            vec![10, 20],        // avg = 15.0, above avg: 20
            vec![7, 8, 9],       // avg = 8.0, above avg: 9
        ];
        let mut result = elements_above_average(&data);
        result.sort(); // Sort for consistent comparison
        assert_eq!(result, vec![4, 5, 9, 20]);
    }

    #[test]
    fn test_elements_above_average_no_elements() {
        let data = vec![
            vec![1, 1, 1], // avg = 1.0, no elements above
            vec![5, 5],    // avg = 5.0, no elements above
        ];
        let result = elements_above_average(&data);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_elements_above_average_empty_data() {
        let data: Vec<Vec<i32>> = vec![];
        let result = elements_above_average(&data);
        assert_eq!(result, vec![]);
    }
}