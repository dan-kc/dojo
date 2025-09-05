// Sum of Squares of Even Numbers
//
// Learning objectives:
// - Understanding iterator performance characteristics
// - Comparing iterator vs loop approaches
// - Using iterator methods for optimal performance
//
// cargo test --bin sum_squares_evens

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
}

fn main() {
    println!("Run tests with: cargo test --bin sum_squares_evens");
}