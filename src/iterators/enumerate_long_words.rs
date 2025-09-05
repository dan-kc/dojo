// Enumerate Long Words
//
// Learning objectives:
// - Combining skip(), enumerate(), filter(), and map()
// - Understanding iterator chain order
// - Working with indices after transformations
//
// cargo test --bin enumerate_long_words

/// Given a vector of words, return a vector of tuples containing
/// (index, word, character_count) for words longer than 4 characters.
/// Skip the first 2 words in the original vector.
pub fn enumerate_long_words(words: Vec<String>) -> Vec<(usize, String, usize)> {
    todo!("Combine skip(), enumerate(), filter(), and map()")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate_long_words() {
        let words = vec![
            "hi".to_string(),
            "go".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "programming".to_string(),
            "rust".to_string(),
        ];
        let result = enumerate_long_words(words);
        // Skips "hi", "go", then enumerates from index 0 for remaining
        // Filters for words > 4 chars: "hello"(5), "world"(5), "programming"(11)
        assert_eq!(result, vec![
            (0, "hello".to_string(), 5),
            (1, "world".to_string(), 5),
            (2, "programming".to_string(), 11),
        ]);
    }

    #[test]
    fn test_enumerate_long_words_insufficient_data() {
        let words = vec!["hi".to_string()];
        let result = enumerate_long_words(words);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_enumerate_long_words_no_long_words() {
        let words = vec![
            "skip".to_string(),
            "this".to_string(),
            "hi".to_string(),
            "no".to_string(),
            "long".to_string(),
        ];
        let result = enumerate_long_words(words);
        assert_eq!(result, vec![]); // After skipping 2, no words > 4 chars
    }

    #[test]
    fn test_enumerate_long_words_all_long() {
        let words = vec![
            "skip1".to_string(),
            "skip2".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "coding".to_string(),
        ];
        let result = enumerate_long_words(words);
        assert_eq!(result, vec![
            (0, "hello".to_string(), 5),
            (1, "world".to_string(), 5),
            (2, "coding".to_string(), 6),
        ]);
    }

    #[test]
    fn test_enumerate_long_words_empty() {
        let words = vec![];
        let result = enumerate_long_words(words);
        assert_eq!(result, vec![]);
    }
}

fn main() {
    println!("Run tests with: cargo test --bin enumerate_long_words");
}