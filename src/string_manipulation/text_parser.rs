// String Parsing and Validation Practice
//
// Learning Objectives:
// - Parse structured text data into key-value pairs
// - Handle different string types (String vs &str)
// - Implement proper error handling for parsing operations
// - Use iterator patterns for efficient string processing
// - Validate input format and handle edge cases
//
// Run tests with: cargo test --lib string_manipulation::text_parser

/// Parses a configuration-style text into key-value pairs.
///
/// The input format should be "key=value" pairs separated by newlines.
/// Keys and values may contain whitespace, which should be trimmed.
/// Empty lines and lines starting with '#' should be ignored.
///
/// Returns a Result containing either a HashMap of parsed pairs or an error message
/// for invalid format (missing '=' or empty keys after trimming).
///
/// # Examples
/// ```
/// let config = "name = John Doe\nage=25\n# comment\nemail=john@example.com";
/// let result = parse_key_value_pairs(config)?;
/// assert_eq!(result.get("name"), Some(&"John Doe".to_string()));
/// ```
pub fn parse_key_value_pairs(
    input: &str,
) -> Result<std::collections::HashMap<String, String>, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_basic_parsing() {
        let input = "name=John\nage=25\nemail=john@example.com";
        let result = parse_key_value_pairs(input).unwrap();

        assert_eq!(result.get("name"), Some(&"John".to_string()));
        assert_eq!(result.get("age"), Some(&"25".to_string()));
        assert_eq!(result.get("email"), Some(&"john@example.com".to_string()));
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_whitespace_trimming() {
        let input = "  name  =  John Doe  \n  age=25\n  city = New York  ";
        let result = parse_key_value_pairs(input).unwrap();

        assert_eq!(result.get("name"), Some(&"John Doe".to_string()));
        assert_eq!(result.get("age"), Some(&"25".to_string()));
        assert_eq!(result.get("city"), Some(&"New York".to_string()));
    }

    #[test]
    fn test_comments_and_empty_lines() {
        let input = "name=John\n# This is a comment\n\nage=25\n# Another comment\n\nemail=john@example.com\n";
        let result = parse_key_value_pairs(input).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result.get("name"), Some(&"John".to_string()));
        assert_eq!(result.get("age"), Some(&"25".to_string()));
        assert_eq!(result.get("email"), Some(&"john@example.com".to_string()));
    }

    #[test]
    fn test_error_cases() {
        // Missing equals sign
        let result = parse_key_value_pairs("name John\nage=25");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing '='"));

        // Empty key
        let result = parse_key_value_pairs("=value\nname=John");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty key"));

        // Only whitespace key
        let result = parse_key_value_pairs("   =value\nname=John");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty key"));
    }
}
