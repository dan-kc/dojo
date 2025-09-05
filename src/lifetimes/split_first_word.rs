// Split First Word
//
// Learning objectives:
// - Understanding lifetime elision rules
// - Working with string slices and lifetimes
// - Returning multiple references with same lifetime
//
// cargo test --bin split_first_word

/// Create a function that takes a string slice and returns a tuple containing
/// the first word and the rest of the string. If there's no space, return
/// the entire string as the first element and an empty string as the second.
/// Pay attention to lifetime requirements for the returned references.
pub fn split_first_word(s: &str) -> (&str, &str) {
    todo!("Implement with correct lifetime handling")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_split_first_word_multiple_spaces() {
        let text = "hello  world  rust";
        let (first, rest) = split_first_word(text);
        assert_eq!(first, "hello");
        assert_eq!(rest, " world  rust"); // Preserves extra spaces
    }

    #[test]
    fn test_split_first_word_trailing_space() {
        let text = "hello ";
        let (first, rest) = split_first_word(text);
        assert_eq!(first, "hello");
        assert_eq!(rest, "");
    }
}

fn main() {
    println!("Run tests with: cargo test --bin split_first_word");
}