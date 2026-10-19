// cargo test --lib string_manipulation::word_counter::

//! Words are maximal runs of Unicode alphabetic characters. Count frequencies case-insensitively
//! using Unicode lowercase; break most-common ties lexicographically. Count characters as Unicode
//! scalars; character_count_no_spaces excludes all Unicode whitespace. Line count follows str::lines.
//! Average original word length in scalars is zero when there are no words. Count punctuation and
//! whitespace in total characters. Do not normalize accents or treat graphemes as single scalars.

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

        assert_eq!(stats.word_count, 9);
        assert_eq!(stats.line_count, 3);
        assert_eq!(stats.unique_words, 7);
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
        assert_eq!(stats.character_count, 17);
    }
    #[test]
    fn ties_unicode_lengths() {
        let s = count_words("雪 café\tcafé 雪");
        assert_eq!(s.most_common_word.as_deref(), Some("café"));
        assert_eq!(s.average_word_length, 2.5);
        assert_eq!(s.character_count_no_spaces, 10);
    }
}
