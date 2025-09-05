// Find Longest String
//
// Learning objectives:
// - Lifetime relationships with collections
// - Returning references from slices
// - Understanding Option with lifetimes
//
// cargo test --bin find_longest_string

/// Implement a function that takes a vector of string slices and returns
/// the one with the maximum length. Return None if the vector is empty.
/// Consider the lifetime relationship between input and output.
pub fn find_longest_string<'a>(strings: &'a [&'a str]) -> Option<&'a str> {
    strings.iter().max_by_key(|s| s.len()).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_longest_string() {
        let strings = ["hi", "hello", "world", "programming"];
        assert_eq!(find_longest_string(&strings), Some("programming"));
    }

    #[test]
    fn test_find_longest_string_multiple_same_length() {
        let strings = ["hello", "world", "rust!"];
        // Should return first occurrence of max length
        assert_eq!(find_longest_string(&strings), Some("hello"));
    }

    #[test]
    fn test_find_longest_string_empty() {
        let strings: [&str; 0] = [];
        assert_eq!(find_longest_string(&strings), None);
    }

    #[test]
    fn test_find_longest_string_single() {
        let strings = ["rust"];
        assert_eq!(find_longest_string(&strings), Some("rust"));
    }

    #[test]
    fn test_find_longest_string_with_empty_string() {
        let strings = ["", "a", "ab", "abc"];
        assert_eq!(find_longest_string(&strings), Some("abc"));
    }

    #[test]
    fn test_find_longest_string_all_empty() {
        let strings = ["", "", ""];
        // Should return first empty string
        assert_eq!(find_longest_string(&strings), Some(""));
    }
}

fn main() {
    println!("Run tests with: cargo test --bin find_longest_string");
}