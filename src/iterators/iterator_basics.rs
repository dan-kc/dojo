// Iterator Basics Practice
// 
// Learning objectives:
// - Understanding iterator creation from collections
// - Basic iterator methods: next(), collect()
// - Iterator vs IntoIterator traits
// - Lazy evaluation concepts
//
// cargo test --lib iterators::iterator_basics

/// Create a function that takes a vector of integers and returns
/// the sum of all even numbers using iterators.
pub fn sum_evens(numbers: Vec<i32>) -> i32 {
    todo!("Implement using iterators to sum only even numbers")
}

/// Create a function that takes a slice of strings and returns
/// a vector containing only strings that are longer than 3 characters.
/// Use iterator methods for the filtering and collection.
pub fn filter_long_strings(strings: &[&str]) -> Vec<String> {
    todo!("Implement using iterators to filter and collect")
}

/// Implement a function that takes a range and returns the first 5
/// numbers that are divisible by 3, collected into a vector.
pub fn first_five_divisible_by_three(start: i32, end: i32) -> Vec<i32> {
    todo!("Use iterator methods with take() and filter()")
}

/// Create a function that converts a vector of integers to a vector of their
/// string representations, but only for positive numbers.
pub fn positive_numbers_to_strings(numbers: Vec<i32>) -> Vec<String> {
    todo!("Chain filter and map operations")
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
    fn test_filter_long_strings() {
        let strings = &["hi", "hello", "rust", "programming", "a", "test"];
        let result = filter_long_strings(strings);
        assert_eq!(result, vec!["hello", "rust", "programming", "test"]);
    }

    #[test]
    fn test_filter_long_strings_empty_input() {
        let strings = &[];
        let result = filter_long_strings(strings);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_filter_long_strings_all_short() {
        let strings = &["hi", "go", "a", "be"];
        let result = filter_long_strings(strings);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_first_five_divisible_by_three() {
        let result = first_five_divisible_by_three(1, 50);
        assert_eq!(result, vec![3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_first_five_divisible_by_three_limited_range() {
        let result = first_five_divisible_by_three(10, 15);
        assert_eq!(result, vec![12]); // Only one number in range
    }

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
}