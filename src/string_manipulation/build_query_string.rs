// cargo test --lib string_manipulation::build_query_string::

//! Build a query string without a leading ?. Omit None values; retain Some(""). Sort by the
//! original unencoded key, then percent-encode keys and values, joining key=value pairs with &.
//! The supplied encoder is infrastructure; this exercise concerns optional map entries and ordered output.

pub fn build_query_string(params: &std::collections::HashMap<String, Option<String>>) -> String {
    todo!()
}

fn url_encode(input: &str) -> String {
    use std::fmt::Write;
    let mut result = String::new();
    for b in input.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            result.push(b as char);
        } else {
            write!(result, "%{b:02X}").unwrap();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
        assert!(result.contains("http%3A%2F%2Fexample.com%3Ffoo%3Dbar"));
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
