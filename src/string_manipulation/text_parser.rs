// cargo test --lib string_manipulation::text_parser::

//! Parse newline-separated key=value entries. Trim each key and value, skip blank lines and
//! lines whose first non-whitespace character is #, and split at the first = only. Empty values
//! are valid; duplicate keys take the last value. On the first invalid line return an error
//! containing its 1-based line number and either "missing '='" or "empty key".

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
    #[test]
    fn duplicates_and_error_lines() {
        let m = parse_key_value_pairs(" # comment\nx=old\nx=a=b\ny=\n").unwrap();
        assert_eq!(m["x"], "a=b");
        assert_eq!(m["y"], "");
        assert_eq!(
            parse_key_value_pairs("# comment\n\nbad").unwrap_err(),
            "Line 3: missing '='"
        );
    }
}
