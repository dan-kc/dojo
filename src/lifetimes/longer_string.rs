// Lifetime Annotations for String Comparison Practice
//
// Learning objectives:
// - Understanding lifetime annotations syntax
// - Resolving lifetime conflicts in function signatures
// - Working with multiple input lifetimes
// - Understanding the relationship between input and output lifetimes
//
// cargo test --lib lifetimes::longer_string

/// Fix the lifetime annotations in this function that returns the longer of two string slices.
/// The returned reference should be valid as long as both input references are valid.
///
/// The function should:
/// - Take two string slice references as input
/// - Return a reference to the longer string
/// - If strings are equal length, return the first one
/// - Use proper lifetime annotations to ensure memory safety
///
/// Lifetime requirements:
/// - Both input strings must have the same lifetime 'a
/// - The returned reference has the same lifetime 'a
/// - This ensures the returned reference is valid as long as both inputs are valid
///
/// # Arguments
/// * `s1` - First string slice to compare
/// * `s2` - Second string slice to compare
///
/// # Returns
/// A reference to the longer string slice
///
/// # Example
/// ```
/// let s1 = "hello";
/// let s2 = "hi";
/// assert_eq!(longer_string(s1, s2), "hello");
/// ```
pub fn longer_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    todo!("Add appropriate lifetime annotations and implement comparison logic")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer_string_first_longer() {
        let s1 = "hello world";
        let s2 = "hi";
        assert_eq!(longer_string(s1, s2), "hello world");
    }

    #[test]
    fn test_longer_string_second_longer() {
        let s1 = "hi";
        let s2 = "hello world";
        assert_eq!(longer_string(s1, s2), "hello world");
    }

    #[test]
    fn test_longer_string_equal_length() {
        let s1 = "hello";
        let s2 = "world";
        // Should return first when equal
        assert_eq!(longer_string(s1, s2), "hello");
    }

    #[test]
    fn test_longer_string_empty_strings() {
        let s1 = "";
        let s2 = "something";
        assert_eq!(longer_string(s1, s2), "something");
    }

    #[test]
    fn test_longer_string_both_empty() {
        let s1 = "";
        let s2 = "";
        assert_eq!(longer_string(s1, s2), "");
    }

    #[test]
    fn test_longer_string_unicode() {
        let s1 = "café";
        let s2 = "restaurant";
        assert_eq!(longer_string(s1, s2), "restaurant");
    }

    #[test]
    fn test_longer_string_lifetime_scope() {
        let result;
        let s1 = "outer scope";
        {
            let s2 = "inner";
            result = longer_string(s1, s2);
            // result is valid here because both s1 and s2 are valid
            assert_eq!(result, "outer scope");
        }
        // This would fail to compile if lifetimes weren't correct:
        // assert_eq!(result, "outer scope"); // s2 is out of scope
    }
}