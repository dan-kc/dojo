// Character Frequency Counter Practice
//
// Learning objectives:
// - Using HashMap Entry API (or_insert, and_modify)
// - Efficient character counting patterns
// - Understanding Entry API performance benefits
//
// Run with: cargo test hashmap_char_frequencies

/// Use the Entry API to count character frequencies in a string.
/// Implement efficient counting using or_insert and and_modify.
pub fn count_char_frequencies(text: &str) -> std::collections::HashMap<char, usize> {
    todo!("Implement character frequency counting using Entry API")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_char_frequencies() {
        let text = "hello world";
        let freq = count_char_frequencies(text);
        
        assert_eq!(freq.get(&'l'), Some(&3));
        assert_eq!(freq.get(&'o'), Some(&2));
        assert_eq!(freq.get(&'h'), Some(&1));
        assert_eq!(freq.get(&'w'), Some(&1));
        assert_eq!(freq.get(&' '), Some(&1));
        assert_eq!(freq.get(&'z'), None);
        
        // Test empty string
        let empty_freq = count_char_frequencies("");
        assert!(empty_freq.is_empty());
    }

    #[test]
    fn test_count_repeated_chars() {
        let text = "aaaaabbbbccccddd";
        let freq = count_char_frequencies(text);
        
        assert_eq!(freq.get(&'a'), Some(&5));
        assert_eq!(freq.get(&'b'), Some(&4));
        assert_eq!(freq.get(&'c'), Some(&4));
        assert_eq!(freq.get(&'d'), Some(&3));
    }

    #[test]
    fn test_count_unicode_chars() {
        let text = "café naïve résumé";
        let freq = count_char_frequencies(text);
        
        assert_eq!(freq.get(&'é'), Some(&3));
        assert_eq!(freq.get(&'ï'), Some(&1));
        assert_eq!(freq.get(&'ç'), Some(&1));
        assert!(freq.contains_key(&'à'));
    }
}