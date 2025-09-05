// Palindrome Checker Practice
//
// Learning Objectives:
// - Master VecDeque for double-ended queue operations
// - Use efficient removal from both ends for palindrome checking
// - Understand when VecDeque provides performance advantages
// - Practice string processing with specialized collections
//
// Run with: cargo test --bin palindrome_checker

/// Implement a palindrome checker using VecDeque's double-ended capabilities.
/// Remove characters from both ends and compare for efficiency.
pub fn is_palindrome_deque(s: &str) -> bool {
    todo!("Implement palindrome checking using VecDeque")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_deque() {
        assert!(is_palindrome_deque("racecar"));
        assert!(is_palindrome_deque("level"));
        assert!(!is_palindrome_deque("hello"));
        assert!(is_palindrome_deque(""));
        assert!(is_palindrome_deque("a"));
        assert!(is_palindrome_deque("aa"));
        assert!(!is_palindrome_deque("ab"));
        
        // Test with spaces and punctuation
        assert!(is_palindrome_deque("a man a plan a canal panama"));
    }

    #[test]
    fn test_edge_cases() {
        // Test empty string
        assert!(is_palindrome_deque(""));
        
        // Test single character
        assert!(is_palindrome_deque("x"));
    }
}