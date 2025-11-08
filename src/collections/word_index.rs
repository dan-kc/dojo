// cargo test word_index

/// Use HashMap to implement a simple word index for text search.
/// Map words to sets of positions where they appear.
pub fn build_word_index(
    text: &str,
) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_word_index() {
        let text = "the quick brown fox jumps over the lazy dog";
        let index = build_word_index(text);

        let the_positions = index.get("the").unwrap();
        assert!(the_positions.contains(&0)); // First "the"
        assert!(the_positions.contains(&6)); // Second "the"
        assert_eq!(the_positions.len(), 2);

        let fox_positions = index.get("fox").unwrap();
        assert!(fox_positions.contains(&3));
        assert_eq!(fox_positions.len(), 1);

        assert_eq!(index.get("nonexistent"), None);
    }

    #[test]
    fn test_build_word_index_single_word() {
        let text = "hello";
        let index = build_word_index(text);

        assert_eq!(index.len(), 1);
        let hello_positions = index.get("hello").unwrap();
        assert!(hello_positions.contains(&0));
        assert_eq!(hello_positions.len(), 1);
    }

    #[test]
    fn test_build_word_index_repeated_words() {
        let text = "test test test";
        let index = build_word_index(text);

        assert_eq!(index.len(), 1);
        let test_positions = index.get("test").unwrap();
        assert!(test_positions.contains(&0));
        assert!(test_positions.contains(&1));
        assert!(test_positions.contains(&2));
        assert_eq!(test_positions.len(), 3);
    }

    #[test]
    fn test_build_word_index_empty_text() {
        let text = "";
        let index = build_word_index(text);
        assert!(index.is_empty());
    }

    #[test]
    fn test_build_word_index_whitespace_only() {
        let text = "   \t\n  ";
        let index = build_word_index(text);
        assert!(index.is_empty());
    }

    #[test]
    fn test_build_word_index_case_sensitivity() {
        let text = "Hello hello HELLO";
        let index = build_word_index(text);

        // Should treat different cases as different words (or normalize - implementation choice)
        // This test assumes case-sensitive indexing
        if index.contains_key("hello") && index.contains_key("Hello") && index.contains_key("HELLO")
        {
            // Case-sensitive implementation
            assert_eq!(index.len(), 3);
        } else if index.contains_key("hello") && index.len() == 1 {
            // Case-insensitive implementation
            let hello_positions = index.get("hello").unwrap();
            assert_eq!(hello_positions.len(), 3);
        }
    }
}
