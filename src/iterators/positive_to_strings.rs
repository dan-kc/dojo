// Convert Positive Numbers to Strings Practice
//
// Learning objectives:
// - Chaining filter() and map() operations
// - Understanding iterator transformation pipelines
// - Converting between numeric and string types
// - Working with ownership in iterator chains
//
// cargo test --lib iterators::positive_to_strings

/// Create a function that converts a vector of integers to a vector of their
/// string representations, but only for positive numbers.
///
/// The function should:
/// - Filter out zero and negative numbers
/// - Convert remaining positive numbers to strings
/// - Preserve the order of numbers
/// - Use iterator method chaining
///
/// # Arguments
/// * `numbers` - A vector of i32 integers
///
/// # Returns
/// A vector of strings representing only the positive numbers
///
/// # Example
/// ```
/// let numbers = vec![-2, -1, 0, 1, 2, 3];
/// let result = positive_numbers_to_strings(numbers);
/// assert_eq!(result, vec!["1", "2", "3"]);
/// ```
pub fn positive_numbers_to_strings(numbers: Vec<i32>) -> Vec<String> {
    todo!("Chain filter and map operations")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers_to_strings() {
        let numbers = vec![-2, -1, 0, 1, 2, 3];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_positive_numbers_to_strings_all_negative() {
        let numbers = vec![-5, -3, -1];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_positive_numbers_to_strings_empty() {
        let numbers = vec![];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_positive_numbers_to_strings_all_positive() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, vec!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn test_positive_numbers_to_strings_with_zero() {
        let numbers = vec![0, 0, 0, 1];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, vec!["1"]);
    }

    #[test]
    fn test_positive_numbers_to_strings_large_numbers() {
        let numbers = vec![1000, -2000, 3000, 0, 4000];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, vec!["1000", "3000", "4000"]);
    }

    #[test]
    fn test_positive_numbers_to_strings_single_positive() {
        let numbers = vec![42];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, vec!["42"]);
    }

    #[test]
    fn test_positive_numbers_to_strings_single_negative() {
        let numbers = vec![-42];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_positive_numbers_to_strings_alternating() {
        let numbers = vec![-1, 1, -2, 2, -3, 3];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_positive_numbers_to_strings_preserves_order() {
        let numbers = vec![5, -1, 3, -2, 1, 0, 4, 2];
        let result = positive_numbers_to_strings(numbers);
        assert_eq!(result, vec!["5", "3", "1", "4", "2"]);
    }
}