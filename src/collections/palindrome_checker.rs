// cargo test palindrome_checker

/// Implement a palindrome checker.
#[allow(unused_variables)]
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
