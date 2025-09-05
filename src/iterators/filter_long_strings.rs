// Filter Long Strings Using Iterators Practice
//
// Learning objectives:
// - Working with string slices and iterators
// - Using filter() with custom predicates
// - Converting between &str and String
// - Using map() for type transformation
// - Chaining iterator methods
//
// cargo test --lib iterators::filter_long_strings

/// Create a function that takes a slice of strings and returns
/// a vector containing only strings that are longer than 3 characters.
/// Use iterator methods for the filtering and collection.
///
/// The function should:
/// - Iterate through the string slice
/// - Filter strings with length > 3
/// - Convert &str to String for the output vector
/// - Preserve the order of strings
///
/// # Arguments
/// * `strings` - A slice of string slices
///
/// # Returns
/// A vector of owned Strings containing only the strings longer than 3 characters
///
/// # Example
/// ```
/// let strings = &["hi", "hello", "rust", "programming"];
/// let result = filter_long_strings(strings);
/// assert_eq!(result, vec!["hello", "rust", "programming"]);
/// ```
pub fn filter_long_strings(strings: &[&str]) -> Vec<String> {
    todo!("Implement using iterators to filter and collect")
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let strings = &["hi", "go", "a", "be", "it"];
        let result = filter_long_strings(strings);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_filter_long_strings_all_long() {
        let strings = &["hello", "world", "rust", "programming"];
        let result = filter_long_strings(strings);
        assert_eq!(result, vec!["hello", "world", "rust", "programming"]);
    }

    #[test]
    fn test_filter_long_strings_exactly_three() {
        let strings = &["abc", "test", "xyz"];
        let result = filter_long_strings(strings);
        assert_eq!(result, vec!["test"]); // Only "test" is longer than 3
    }

    #[test]
    fn test_filter_long_strings_with_spaces() {
        let strings = &["hi ", " hello", "  ", "test string"];
        let result = filter_long_strings(strings);
        assert_eq!(result, vec![" hello", "test string"]);
    }

    #[test]
    fn test_filter_long_strings_unicode() {
        let strings = &["🦀", "café", "日本", "hello"];
        let result = filter_long_strings(strings);
        assert_eq!(result, vec!["café", "hello"]); // emoji and 日本 are not > 3 chars
    }

    #[test]
    fn test_filter_long_strings_preserves_order() {
        let strings = &["zebra", "apple", "hi", "banana", "no"];
        let result = filter_long_strings(strings);
        assert_eq!(result, vec!["zebra", "apple", "banana"]);
    }
}