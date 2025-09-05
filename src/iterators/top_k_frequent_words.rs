// Top K Frequent Words
//
// Learning objectives:
// - Efficient text processing with iterators
// - Using HashMap with iterator operations
// - Sorting and limiting results efficiently
//
// cargo test --bin top_k_frequent_words

/// Implement an efficient function to find the top K most frequent words
/// in a text, using iterator methods for text processing and HashMap operations.
pub fn top_k_frequent_words(text: &str, k: usize) -> Vec<String> {
    todo!("Use iterators to split, count, and find top K efficiently")
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
        assert_eq!(result, vec![]);
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

fn main() {
    println!("Run tests with: cargo test --bin top_k_frequent_words");
}