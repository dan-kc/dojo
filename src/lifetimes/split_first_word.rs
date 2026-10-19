// cargo test --lib lifetimes::split_first_word::

//! Split at the first ASCII space only, removing exactly that one separator. Preserve any further spaces. If absent return (input, ""). Both results borrow the input.

/// Create a function that takes a string slice and returns a tuple containing
/// the first word and the rest of the string. If there's no space, return
/// the entire string as the first element and an empty string as the second.
/// Pay attention to lifetime requirements for the returned references.
pub fn split_first_word(s: &str) -> (&str, &str) {
    todo!()
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
