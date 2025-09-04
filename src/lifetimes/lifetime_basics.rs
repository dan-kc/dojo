// Lifetime Basics Practice
//
// Learning objectives:
// - Understanding lifetime annotations syntax
// - Lifetime elision rules
// - Resolving basic lifetime conflicts
// - Function signature lifetime requirements
//
// cargo test --lib lifetimes::lifetime_basics

/// Fix the lifetime annotations in this function that returns the longer of two string slices.
/// The returned reference should be valid as long as both input references are valid.
pub fn longer_string(s1: &str, s2: &str) -> &str {
    todo!("Add appropriate lifetime annotations")
}

/// Create a function that takes a string slice and returns a tuple containing
/// the first word and the rest of the string. If there's no space, return
/// the entire string as the first element and an empty string as the second.
/// Pay attention to lifetime requirements for the returned references.
pub fn split_first_word(s: &str) -> (&str, &str) {
    todo!("Implement with correct lifetime handling")
}

/// Implement a function that takes a vector of string slices and returns
/// the one with the maximum length. Return None if the vector is empty.
/// Consider the lifetime relationship between input and output.
pub fn find_longest_string(strings: &[&str]) -> Option<&str> {
    todo!("Handle lifetimes when returning reference from collection")
}

/// Create a function that takes two string slices and returns a String
/// containing the concatenation of the longer slice repeated twice.
/// This demonstrates when you don't need lifetime annotations.
pub fn repeat_longer_owned(s1: &str, s2: &str) -> String {
    todo!("Return owned String, no lifetime annotations needed")
}

/// Fix this struct definition and its method to properly handle lifetimes.
/// The struct should hold a reference to a string slice.
pub struct TextHolder {
    todo!("Add lifetime parameter and field")
}

impl TextHolder {
    /// Create a new TextHolder with the given text reference
    pub fn new(text: &str) -> TextHolder {
        todo!("Implement constructor with proper lifetime handling")
    }
    
    /// Return the held text
    pub fn get_text(&self) -> &str {
        todo!("Return the stored text reference")
    }
    
    /// Return the first n characters of the held text
    pub fn get_prefix(&self, n: usize) -> &str {
        todo!("Return prefix with correct lifetime relationship")
    }
}

/// Implement a function that demonstrates lifetime elision.
/// This function should take a string slice and return a reference to
/// the part of the string after the first occurrence of a character.
/// Return the original string if the character is not found.
pub fn after_char(s: &str, c: char) -> &str {
    todo!("Implement using lifetime elision rules")
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
    fn test_split_first_word_with_space() {
        let text = "hello world rust";
        let (first, rest) = split_first_word(text);
        assert_eq!(first, "hello");
        assert_eq!(rest, "world rust");
    }

    #[test]
    fn test_split_first_word_no_space() {
        let text = "hello";
        let (first, rest) = split_first_word(text);
        assert_eq!(first, "hello");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_split_first_word_empty() {
        let text = "";
        let (first, rest) = split_first_word(text);
        assert_eq!(first, "");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_split_first_word_only_space() {
        let text = " world";
        let (first, rest) = split_first_word(text);
        assert_eq!(first, "");
        assert_eq!(rest, "world");
    }

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
    fn test_repeat_longer_owned() {
        let s1 = "hi";
        let s2 = "hello";
        assert_eq!(repeat_longer_owned(s1, s2), "hellohello");
    }

    #[test]
    fn test_repeat_longer_owned_first_longer() {
        let s1 = "programming";
        let s2 = "code";
        assert_eq!(repeat_longer_owned(s1, s2), "programmingprogramming");
    }

    #[test]
    fn test_text_holder_basic_usage() {
        let text = "hello world";
        let holder = TextHolder::new(text);
        assert_eq!(holder.get_text(), "hello world");
    }

    #[test]
    fn test_text_holder_prefix() {
        let text = "hello world";
        let holder = TextHolder::new(text);
        assert_eq!(holder.get_prefix(5), "hello");
        assert_eq!(holder.get_prefix(20), "hello world"); // Beyond length
        assert_eq!(holder.get_prefix(0), "");
    }

    #[test]
    fn test_text_holder_lifetime_validity() {
        let holder = {
            let text = "temporary";
            TextHolder::new(text)
        }; // text goes out of scope here
        // This test verifies the struct compiles correctly with lifetime annotations
        // In actual usage, the above would cause a compilation error
    }

    #[test]
    fn test_after_char_found() {
        let text = "hello::world";
        assert_eq!(after_char(text, ':'), ":world");
    }

    #[test]
    fn test_after_char_not_found() {
        let text = "hello world";
        assert_eq!(after_char(text, 'x'), "hello world");
    }

    #[test]
    fn test_after_char_at_end() {
        let text = "hello:";
        assert_eq!(after_char(text, ':'), "");
    }

    #[test]
    fn test_after_char_empty_string() {
        let text = "";
        assert_eq!(after_char(text, 'a'), "");
    }
}