// HashSet Spell Checker Practice
//
// Learning objectives:
// - Use HashSet for fast dictionary lookups
// - String processing and text parsing
// - Set-based filtering operations
//
// Run with: cargo test spell_check

/// Implement a simple spell checker using HashSet for dictionary lookup.
/// Return words that are not in the dictionary.
pub fn spell_check(
    text: &str,
    dictionary: &std::collections::HashSet<String>,
) -> std::collections::HashSet<String> {
    todo!("Implement spell checking using HashSet")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_spell_check() {
        let dictionary: HashSet<String> = ["hello", "world", "rust", "programming"]
            .iter().map(|s| s.to_string()).collect();
        
        let text = "hello wrold rust programing test";
        let errors = spell_check(text, &dictionary);
        
        let expected_errors: HashSet<String> = ["wrold", "programing", "test"]
            .iter().map(|s| s.to_string()).collect();
        
        assert_eq!(errors, expected_errors);
        
        // Test with no errors
        let correct_text = "hello world rust programming";
        let no_errors = spell_check(correct_text, &dictionary);
        assert!(no_errors.is_empty());
    }

    #[test]
    fn test_spell_check_case_sensitive() {
        let dictionary: HashSet<String> = ["Hello", "World"]
            .iter().map(|s| s.to_string()).collect();
        
        let text = "hello world Hello World";
        let errors = spell_check(text, &dictionary);
        
        // Should find "hello" and "world" as errors (case mismatch)
        let expected_errors: HashSet<String> = ["hello", "world"]
            .iter().map(|s| s.to_string()).collect();
        
        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn test_spell_check_punctuation() {
        let dictionary: HashSet<String> = ["hello", "world", "test"]
            .iter().map(|s| s.to_string()).collect();
        
        let text = "hello, world! test?";
        let errors = spell_check(text, &dictionary);
        
        // Should handle punctuation correctly - no errors if punctuation is stripped
        assert!(errors.is_empty() || errors.contains("hello,") || errors.contains("world!") || errors.contains("test?"));
    }

    #[test]
    fn test_spell_check_empty_input() {
        let dictionary: HashSet<String> = ["word"].iter().map(|s| s.to_string()).collect();
        
        let empty_text = "";
        let errors = spell_check(empty_text, &dictionary);
        assert!(errors.is_empty());
        
        let whitespace_text = "   \n\t  ";
        let errors = spell_check(whitespace_text, &dictionary);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_spell_check_empty_dictionary() {
        let empty_dict: HashSet<String> = HashSet::new();
        let text = "every word is wrong";
        let errors = spell_check(text, &empty_dict);
        
        let expected_errors: HashSet<String> = ["every", "word", "is", "wrong"]
            .iter().map(|s| s.to_string()).collect();
        
        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn test_spell_check_repeated_words() {
        let dictionary: HashSet<String> = ["correct", "word"]
            .iter().map(|s| s.to_string()).collect();
        
        let text = "correct wrong correct wrong word";
        let errors = spell_check(text, &dictionary);
        
        // Should only contain "wrong" once, even though it appears twice
        let expected_errors: HashSet<String> = ["wrong"].iter().map(|s| s.to_string()).collect();
        
        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn test_spell_check_mixed_content() {
        let dictionary: HashSet<String> = ["the", "quick", "brown", "fox", "jumps", "over", "lazy", "dog"]
            .iter().map(|s| s.to_string()).collect();
        
        let text = "the qwick brown fox jmps over the lazi dog";
        let errors = spell_check(text, &dictionary);
        
        let expected_errors: HashSet<String> = ["qwick", "jmps", "lazi"]
            .iter().map(|s| s.to_string()).collect();
        
        assert_eq!(errors, expected_errors);
    }

    #[test]
    fn test_spell_check_numbers_and_special() {
        let dictionary: HashSet<String> = ["test", "with", "numbers"]
            .iter().map(|s| s.to_string()).collect();
        
        let text = "test with numbers 123 and symbols @#$";
        let errors = spell_check(text, &dictionary);
        
        // Behavior depends on implementation - numbers/symbols might be treated as errors
        // This tests that the function handles non-alphabetic characters gracefully
        assert!(!errors.contains("test"));
        assert!(!errors.contains("with"));
        assert!(!errors.contains("numbers"));
    }

    #[test]
    fn test_spell_check_single_word() {
        let dictionary: HashSet<String> = ["hello"].iter().map(|s| s.to_string()).collect();
        
        let correct_word = "hello";
        let errors = spell_check(correct_word, &dictionary);
        assert!(errors.is_empty());
        
        let incorrect_word = "goodbye";
        let errors = spell_check(incorrect_word, &dictionary);
        let expected_errors: HashSet<String> = ["goodbye"].iter().map(|s| s.to_string()).collect();
        assert_eq!(errors, expected_errors);
    }
}