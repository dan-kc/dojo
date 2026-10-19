// cargo test --lib collections::palindrome_checker::

//! Compare Unicode alphanumeric characters case-insensitively, ignoring all other characters.
//! Apply Unicode lowercase expansion before comparing scalar values. Empty normalized text is a palindrome.

pub fn is_palindrome(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_deque() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("level"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("aa"));
        assert!(!is_palindrome("ab"));

        // Test with spaces and punctuation
        assert!(is_palindrome("a man a, plan a canal panama"));
    }

    #[test]
    fn test_edge_cases() {
        // Test empty string
        assert!(is_palindrome(""));

        // Test single character
        assert!(is_palindrome("x"));
    }
}

#[cfg(test)]
mod unicode_tests {
    use super::*;
    #[test]
    fn ignores_case_and_punctuation() {
        assert!(is_palindrome("A man, a plan, a canal: Panama!"));
        assert!(!is_palindrome("Rust 12"));
    }
    #[test]
    fn unicode_and_empty_normalization() {
        assert!(is_palindrome("Été"));
        assert!(is_palindrome("!? "));
    }
}
