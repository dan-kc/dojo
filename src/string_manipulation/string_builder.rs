// String Building and Query Construction Practice
//
// Learning Objectives:
// - Build strings efficiently using various methods
// - Understand memory implications of string operations
// - Handle URL encoding and query parameter construction
// - Practice with iterator patterns and string formatting
// - Implement builder pattern for complex string construction
//
// Run tests with: cargo test --lib string_manipulation::string_builder

use std::collections::HashMap;

/// Builds a URL query string from a HashMap of parameters.
///
/// The function should:
/// 1. URL-encode parameter names and values properly
/// 2. Handle special characters (&, =, ?, #, space, etc.)
/// 3. Sort parameters alphabetically by key for consistent output
/// 4. Handle empty values (include key with empty value: "key=")
/// 5. Skip None values but include empty string values
/// 6. Use efficient string building techniques
/// 7. Return the query string without the leading '?'
///
/// URL encoding rules:
/// - Spaces become '%20' (or '+' in form encoding, but use '%20' for this exercise)
/// - '&' becomes '%26'
/// - '=' becomes '%3D'
/// - Special characters should be percent-encoded
/// - Alphanumeric characters, hyphens, underscores, periods, and tildes are safe
///
/// # Arguments
/// * `params` - HashMap where keys are parameter names and values are optional parameter values
///
/// # Returns
/// A properly formatted and encoded query string
///
/// # Examples
/// ```
/// let mut params = HashMap::new();
/// params.insert("name".to_string(), Some("John Doe".to_string()));
/// params.insert("age".to_string(), Some("30".to_string()));
/// let query = build_query_string(&params);
/// assert_eq!(query, "age=30&name=John%20Doe");
/// ```
pub fn build_query_string(params: &HashMap<String, Option<String>>) -> String {
    todo!()
}

/// Helper function to URL-encode a string
///
/// This function should properly encode special characters according to URL encoding rules.
/// For learning purposes, implement this without using external crates.
fn url_encode(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_query_string() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), Some("John".to_string()));
        params.insert("age".to_string(), Some("30".to_string()));

        let result = build_query_string(&params);

        // Should be sorted alphabetically by key
        assert_eq!(result, "age=30&name=John");
    }

    #[test]
    fn test_url_encoding() {
        let mut params = HashMap::new();
        params.insert("message".to_string(), Some("Hello World!".to_string()));
        params.insert("email".to_string(), Some("user@example.com".to_string()));

        let result = build_query_string(&params);

        assert!(result.contains("message=Hello%20World%21"));
        assert!(result.contains("email=user%40example.com"));
    }

    #[test]
    fn test_special_characters() {
        let mut params = HashMap::new();
        params.insert("query".to_string(), Some("name=John&age=30".to_string()));
        params.insert(
            "redirect".to_string(),
            Some("http://example.com?foo=bar".to_string()),
        );

        let result = build_query_string(&params);

        // & and = should be encoded in values
        assert!(result.contains("name%3DJohn%26age%3D30"));
        assert!(result.contains("http%3A//example.com%3Ffoo%3Dbar"));
    }

    #[test]
    fn test_empty_values() {
        let mut params = HashMap::new();
        params.insert("empty".to_string(), Some("".to_string()));
        params.insert("name".to_string(), Some("John".to_string()));

        let result = build_query_string(&params);

        assert_eq!(result, "empty=&name=John");
    }

    #[test]
    fn test_none_values() {
        let mut params = HashMap::new();
        params.insert("skip_me".to_string(), None);
        params.insert("include_me".to_string(), Some("value".to_string()));

        let result = build_query_string(&params);

        assert_eq!(result, "include_me=value");
        assert!(!result.contains("skip_me"));
    }

    #[test]
    fn test_sorting() {
        let mut params = HashMap::new();
        params.insert("zebra".to_string(), Some("last".to_string()));
        params.insert("alpha".to_string(), Some("first".to_string()));
        params.insert("beta".to_string(), Some("second".to_string()));

        let result = build_query_string(&params);

        assert_eq!(result, "alpha=first&beta=second&zebra=last");
    }

    #[test]
    fn test_url_encode_helper() {
        assert_eq!(url_encode("hello world"), "hello%20world");
        assert_eq!(url_encode("user@example.com"), "user%40example.com");
        assert_eq!(
            url_encode("name=value&other=data"),
            "name%3Dvalue%26other%3Ddata"
        );
        assert_eq!(url_encode("100%"), "100%25");
        assert_eq!(url_encode("safe-chars_123.~"), "safe-chars_123.~");
    }

    #[test]
    fn test_empty_params() {
        let params = HashMap::new();
        let result = build_query_string(&params);
        assert_eq!(result, "");
    }

    #[test]
    fn test_complex_encoding() {
        let mut params = HashMap::new();
        params.insert("unicode".to_string(), Some("café naïve".to_string()));
        params.insert("symbols".to_string(), Some("$#@!*()[]{}".to_string()));

        let result = build_query_string(&params);

        // Should properly encode Unicode and symbols
        assert!(result.contains("symbols="));
        assert!(result.contains("unicode="));
        assert!(result.contains("%"));
    }

    #[test]
    fn test_numeric_and_boolean_like_values() {
        let mut params = HashMap::new();
        params.insert("count".to_string(), Some("42".to_string()));
        params.insert("enabled".to_string(), Some("true".to_string()));
        params.insert("ratio".to_string(), Some("3.14159".to_string()));

        let result = build_query_string(&params);

        assert_eq!(result, "count=42&enabled=true&ratio=3.14159");
    }

    #[test]
    fn test_single_parameter() {
        let mut params = HashMap::new();
        params.insert("single".to_string(), Some("value".to_string()));

        let result = build_query_string(&params);

        assert_eq!(result, "single=value");
    }
}
