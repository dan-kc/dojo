// Word Counting and Text Analysis Practice
//
// Learning Objectives:
// - Analyze text content and extract statistics
// - Use iterator patterns for efficient text processing
// - Handle Unicode text correctly
// - Implement custom data structures
// - Practice with String methods and character classification
//
// Run tests with: cargo test --lib string_manipulation::word_counter

/// Statistics about analyzed text
#[derive(Debug, PartialEq)]
pub struct WordStats {
    pub word_count: usize,
    pub unique_words: usize,
    pub character_count: usize,
    pub character_count_no_spaces: usize,
    pub line_count: usize,
    pub average_word_length: f64,
    pub most_common_word: Option<String>,
}

/// Analyzes text and returns comprehensive word statistics.
///
/// Words are defined as sequences of alphabetic characters separated by non-alphabetic characters.
/// The analysis should be case-insensitive for counting purposes.
///
/// # Arguments
/// * `text` - The input text to analyze
///
/// # Returns
/// A WordStats struct containing various text statistics
///
/// # Examples
/// ```
/// let stats = count_words("Hello world! Hello again.");
/// assert_eq!(stats.word_count, 4);
/// assert_eq!(stats.unique_words, 3);
/// assert_eq!(stats.most_common_word, Some("hello".to_string()));
/// ```
pub fn count_words(text: &str) -> WordStats {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_word_counting() {
        let text = "Hello world hello";
        let stats = count_words(text);

        assert_eq!(stats.word_count, 3);
        assert_eq!(stats.unique_words, 2);
        assert_eq!(stats.character_count, 17);
        assert_eq!(stats.character_count_no_spaces, 15);
        assert_eq!(stats.line_count, 1);
        assert_eq!(stats.most_common_word, Some("hello".to_string()));
        assert!((stats.average_word_length - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_multiline_text() {
        let text = "First line\nSecond line with more words\nThird line";
        let stats = count_words(text);

        assert_eq!(stats.word_count, 8);
        assert_eq!(stats.line_count, 3);
        assert!(stats.unique_words <= 8);
    }

    #[test]
    fn test_punctuation_handling() {
        let text = "Hello, world! How are you? I'm fine, thanks.";
        let stats = count_words(text);

        // Should count: Hello, world, How, are, you, I, m, fine, thanks
        assert_eq!(stats.word_count, 9);
        // Note: "I'm" should be split into "I" and "m"
    }

    #[test]
    fn test_case_insensitivity() {
        let text = "Hello HELLO hello HeLLo";
        let stats = count_words(text);

        assert_eq!(stats.word_count, 4);
        assert_eq!(stats.unique_words, 1);
        assert_eq!(stats.most_common_word, Some("hello".to_string()));
    }

    #[test]
    fn test_empty_and_whitespace() {
        let empty_stats = count_words("");
        assert_eq!(empty_stats.word_count, 0);
        assert_eq!(empty_stats.unique_words, 0);
        assert_eq!(empty_stats.character_count, 0);
        assert_eq!(empty_stats.line_count, 0);
        assert_eq!(empty_stats.most_common_word, None);
        assert_eq!(empty_stats.average_word_length, 0.0);

        let whitespace_stats = count_words("   \n\n  \t  ");
        assert_eq!(whitespace_stats.word_count, 0);
        assert_eq!(whitespace_stats.line_count, 3);
        assert!(whitespace_stats.character_count > 0);
    }

    #[test]
    fn test_unicode_text() {
        let text = "café naïve résumé";
        let stats = count_words(text);

        assert_eq!(stats.word_count, 3);
        assert_eq!(stats.unique_words, 3);
        // Should handle Unicode characters properly
        assert!(stats.character_count >= 15);
    }
}
