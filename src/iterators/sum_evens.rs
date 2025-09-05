// Sum Even Numbers Using Iterators Practice
// 
// Learning objectives:
// - Understanding iterator creation from collections
// - Using filter() to select specific elements
// - Using sum() for aggregation
// - Working with closures in iterator methods
//
// cargo test --lib iterators::sum_evens

/// Create a function that takes a vector of integers and returns
/// the sum of all even numbers using iterators.
///
/// The function should:
/// - Iterate through the vector
/// - Filter only even numbers (divisible by 2)
/// - Sum the filtered numbers
/// - Return 0 for empty vectors or vectors with no even numbers
///
/// # Arguments
/// * `numbers` - A vector of i32 integers
///
/// # Returns
/// The sum of all even numbers in the vector
///
/// # Example
/// ```
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// assert_eq!(sum_evens(numbers), 12); // 2 + 4 + 6
/// ```
pub fn sum_evens(numbers: Vec<i32>) -> i32 {
    todo!("Implement using iterators to sum only even numbers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_evens_with_mixed_numbers() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum_evens(numbers), 30); // 2 + 4 + 6 + 8 + 10
    }

    #[test]
    fn test_sum_evens_with_no_evens() {
        let numbers = vec![1, 3, 5, 7, 9];
        assert_eq!(sum_evens(numbers), 0);
    }

    #[test]
    fn test_sum_evens_with_empty_vector() {
        let numbers = vec![];
        assert_eq!(sum_evens(numbers), 0);
    }

    #[test]
    fn test_sum_evens_all_even() {
        let numbers = vec![2, 4, 6, 8, 10];
        assert_eq!(sum_evens(numbers), 30);
    }

    #[test]
    fn test_sum_evens_with_negative_numbers() {
        let numbers = vec![-4, -3, -2, -1, 0, 1, 2, 3, 4];
        assert_eq!(sum_evens(numbers), 0); // -4 + -2 + 0 + 2 + 4
    }

    #[test]
    fn test_sum_evens_single_even() {
        let numbers = vec![42];
        assert_eq!(sum_evens(numbers), 42);
    }

    #[test]
    fn test_sum_evens_single_odd() {
        let numbers = vec![41];
        assert_eq!(sum_evens(numbers), 0);
    }

    #[test]
    fn test_sum_evens_with_zeros() {
        let numbers = vec![0, 0, 0, 1, 2];
        assert_eq!(sum_evens(numbers), 2); // 0 + 0 + 0 + 2
    }
}