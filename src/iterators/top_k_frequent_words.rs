// cargo test --lib iterators::top_k_frequent_words::

//! Words are Unicode-whitespace-separated tokens, preserving case and punctuation. Sort by decreasing frequency, then lexicographically ascending for ties. Return at most k distinct words; k=0 returns empty.

/// Implement an efficient function to find the top K most frequent words
/// in a text, using iterator methods for text processing and HashMap operations.
pub fn top_k_frequent_words(text: &str, k: usize) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent_words() {
        let text = "the quick brown fox jumps over the lazy dog the fox is quick";
        let result = top_k_frequent_words(text, 3);
        // "the" appears 3 times, "quick" and "fox" appear 2 times each
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "the");
        // Note: "quick" and "fox" could be in either order since they have the same frequency
        assert!(result.contains(&"quick".to_string()));
        assert!(result.contains(&"fox".to_string()));
    }

    #[test]
    fn test_top_k_frequent_words_empty() {
        let text = "";
        let result = top_k_frequent_words(text, 3);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_top_k_frequent_words_k_larger_than_unique() {
        let text = "hello world";
        let result = top_k_frequent_words(text, 5);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"hello".to_string()));
        assert!(result.contains(&"world".to_string()));
    }
}
