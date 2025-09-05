// After Char
//
// Learning objectives:
// - Lifetime elision in practice
// - String searching and slicing
// - Working with character positions
//
// cargo test --bin after_char

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

    #[test]
    fn test_after_char_first_char() {
        let text = ":hello";
        assert_eq!(after_char(text, ':'), "hello");
    }

    #[test]
    fn test_after_char_multiple_occurrences() {
        let text = "a:b:c:d";
        assert_eq!(after_char(text, ':'), "b:c:d"); // First occurrence only
    }

    #[test]
    fn test_after_char_unicode() {
        let text = "hello世world";
        assert_eq!(after_char(text, '世'), "world");
    }
}

fn main() {
    println!("Run tests with: cargo test --bin after_char");
}