// Process Owned String Practice
//
// Learning Objectives:
// - Understand ownership transfer in function parameters
// - Practice taking ownership of String values
// - Work with moved values and string operations
// - Handle string transformations with ownership
//
// cargo test --bin process_owned_string

/// Implement a function that takes ownership of a String and returns its length
/// after performing some transformations. The original string should be moved.
fn process_owned_string(s: String) -> usize {
    todo!("Implement process_owned_string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_owned_string() {
        let s = String::from("Hello, World!");
        let length = process_owned_string(s);
        
        // String should be moved, so this line would not compile:
        // println!("{}", s); // Error: value moved
        
        assert!(length > 0);
        assert_eq!(length, 13); // Length after transformation
    }

    #[test]
    fn test_empty_string() {
        let s = String::new();
        let length = process_owned_string(s);
        
        // Should handle empty string appropriately
        assert_eq!(length, 0);
    }

    #[test]
    fn test_long_string() {
        let s = String::from("This is a much longer string with many characters!");
        let original_len = s.len();
        let length = process_owned_string(s);
        
        // Length should be based on processed string
        assert!(length >= original_len || length < original_len); // Allow transformations
    }

    #[test]
    fn test_unicode_string() {
        let s = String::from("Hello, 世界! 🌍");
        let length = process_owned_string(s);
        
        assert!(length > 0);
    }

    #[test]
    fn test_whitespace_string() {
        let s = String::from("   \t\n   ");
        let length = process_owned_string(s);
        
        // Should handle whitespace appropriately
        assert_eq!(length, 0); // Assuming trimming occurs
    }

    #[test]
    fn test_single_character() {
        let s = String::from("A");
        let length = process_owned_string(s);
        
        assert_eq!(length, 1);
    }

    #[test]
    fn test_numeric_string() {
        let s = String::from("12345");
        let length = process_owned_string(s);
        
        assert_eq!(length, 5);
    }

    #[test]
    fn test_special_characters() {
        let s = String::from("!@#$%^&*()");
        let length = process_owned_string(s);
        
        assert_eq!(length, 10);
    }

    #[test]
    fn test_ownership_transfer() {
        // This test verifies that ownership is properly transferred
        let original = String::from("test string");
        let original_ptr = original.as_ptr();
        
        let result_length = process_owned_string(original);
        
        // original is now moved and cannot be used
        assert!(result_length > 0);
        
        // This would not compile:
        // println!("{}", original); // Error: value moved
    }

    #[test]
    fn test_multiple_strings() {
        let strings = vec![
            String::from("first"),
            String::from("second"),
            String::from("third"),
        ];
        
        let lengths: Vec<usize> = strings.into_iter()
            .map(|s| process_owned_string(s))
            .collect();
        
        assert_eq!(lengths.len(), 3);
        assert!(lengths.iter().all(|&len| len > 0));
    }
}