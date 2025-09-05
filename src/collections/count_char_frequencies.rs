// Character Frequency Counting Practice
//
// Learning Objectives:
// - Master HashMap Entry API usage
// - Use or_insert and and_modify methods efficiently
// - Practice character iteration and counting
// - Work with HashMap creation and manipulation
//
// cargo test --bin count_char_frequencies

/// Use the Entry API to count character frequencies in a string.
/// Implement efficient counting using or_insert and and_modify.
fn count_char_frequencies(text: &str) -> std::collections::HashMap<char, usize> {
    todo!("Implement character frequency counting using Entry API")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
    }

    #[test]
    fn test_empty_string() {
        let empty_freq = count_char_frequencies("");
        assert!(empty_freq.is_empty());
    }

    #[test]
    fn test_single_character() {
        let freq = count_char_frequencies("a");
        assert_eq!(freq.len(), 1);
        assert_eq!(freq.get(&'a'), Some(&1));
    }

    #[test]
    fn test_repeated_character() {
        let freq = count_char_frequencies("aaaa");
        assert_eq!(freq.len(), 1);
        assert_eq!(freq.get(&'a'), Some(&4));
    }

    #[test]
    fn test_case_sensitivity() {
        let freq = count_char_frequencies("AaAa");
        assert_eq!(freq.len(), 2);
        assert_eq!(freq.get(&'A'), Some(&2));
        assert_eq!(freq.get(&'a'), Some(&2));
    }

    #[test]
    fn test_unicode_characters() {
        let freq = count_char_frequencies("café");
        assert_eq!(freq.get(&'c'), Some(&1));
        assert_eq!(freq.get(&'a'), Some(&1));
        assert_eq!(freq.get(&'f'), Some(&1));
        assert_eq!(freq.get(&'é'), Some(&1));
        assert_eq!(freq.len(), 4);
    }

    #[test]
    fn test_special_characters() {
        let freq = count_char_frequencies("!@#$%!@");
        assert_eq!(freq.get(&'!'), Some(&2));
        assert_eq!(freq.get(&'@'), Some(&2));
        assert_eq!(freq.get(&'#'), Some(&1));
        assert_eq!(freq.get(&'$'), Some(&1));
        assert_eq!(freq.get(&'%'), Some(&1));
    }

    #[test]
    fn test_whitespace_characters() {
        let freq = count_char_frequencies("a\nb\tc\r d");
        assert_eq!(freq.get(&'a'), Some(&1));
        assert_eq!(freq.get(&'b'), Some(&1));
        assert_eq!(freq.get(&'c'), Some(&1));
        assert_eq!(freq.get(&'d'), Some(&1));
        assert_eq!(freq.get(&'\n'), Some(&1));
        assert_eq!(freq.get(&'\t'), Some(&1));
        assert_eq!(freq.get(&'\r'), Some(&1));
        assert_eq!(freq.get(&' '), Some(&1));
    }

    #[test]
    fn test_numbers_and_letters() {
        let freq = count_char_frequencies("abc123abc");
        assert_eq!(freq.get(&'a'), Some(&2));
        assert_eq!(freq.get(&'b'), Some(&2));
        assert_eq!(freq.get(&'c'), Some(&2));
        assert_eq!(freq.get(&'1'), Some(&1));
        assert_eq!(freq.get(&'2'), Some(&1));
        assert_eq!(freq.get(&'3'), Some(&1));
    }

    #[test]
    fn test_long_string() {
        let text = "the quick brown fox jumps over the lazy dog";
        let freq = count_char_frequencies(text);
        
        // Check some expected frequencies
        assert_eq!(freq.get(&'e'), Some(&3)); // 'e' appears 3 times
        assert_eq!(freq.get(&'o'), Some(&4)); // 'o' appears 4 times
        assert_eq!(freq.get(&' '), Some(&8)); // 8 spaces
        
        // Verify total character count
        let total: usize = freq.values().sum();
        assert_eq!(total, text.len());
    }

    #[test]
    fn test_all_same_character() {
        let freq = count_char_frequencies("zzzzz");
        assert_eq!(freq.len(), 1);
        assert_eq!(freq.get(&'z'), Some(&5));
    }
}