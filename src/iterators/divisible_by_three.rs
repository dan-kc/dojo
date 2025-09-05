// Find First N Numbers Divisible by Three Practice
//
// Learning objectives:
// - Creating iterators from ranges
// - Using filter() with mathematical predicates
// - Using take() to limit results
// - Understanding lazy evaluation
// - Combining multiple iterator methods
//
// cargo test --lib iterators::divisible_by_three

/// Implement a function that takes a range and returns the first 5
/// numbers that are divisible by 3, collected into a vector.
///
/// The function should:
/// - Create an iterator from the range [start, end)
/// - Filter numbers divisible by 3
/// - Take only the first 5 such numbers
/// - Return fewer than 5 if the range doesn't contain enough
/// - Return an empty vector if no numbers match
///
/// # Arguments
/// * `start` - The start of the range (inclusive)
/// * `end` - The end of the range (exclusive)
///
/// # Returns
/// A vector containing up to 5 numbers divisible by 3
///
/// # Example
/// ```
/// let result = first_five_divisible_by_three(1, 20);
/// assert_eq!(result, vec![3, 6, 9, 12, 15]);
/// ```
pub fn first_five_divisible_by_three(start: i32, end: i32) -> Vec<i32> {
    todo!("Use iterator methods with take() and filter()")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_first_five_divisible_by_three_starting_at_zero() {
        let result = first_five_divisible_by_three(0, 20);
        assert_eq!(result, vec![0, 3, 6, 9, 12]);
    }

    #[test]
    fn test_first_five_divisible_by_three_negative_range() {
        let result = first_five_divisible_by_three(-15, 0);
        assert_eq!(result, vec![-15, -12, -9, -6, -3]);
    }

    #[test]
    fn test_first_five_divisible_by_three_no_matches() {
        let result = first_five_divisible_by_three(1, 3);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_first_five_divisible_by_three_exact_five() {
        let result = first_five_divisible_by_three(3, 18);
        assert_eq!(result, vec![3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_first_five_divisible_by_three_more_than_five() {
        let result = first_five_divisible_by_three(3, 100);
        assert_eq!(result, vec![3, 6, 9, 12, 15]); // Should stop at 5
    }

    #[test]
    fn test_first_five_divisible_by_three_empty_range() {
        let result = first_five_divisible_by_three(10, 10);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_first_five_divisible_by_three_reverse_range() {
        let result = first_five_divisible_by_three(10, 5);
        assert_eq!(result, Vec::<i32>::new()); // Range doesn't work backwards
    }

    #[test]
    fn test_first_five_divisible_by_three_large_start() {
        let result = first_five_divisible_by_three(100, 120);
        assert_eq!(result, vec![102, 105, 108, 111, 114]);
    }
}