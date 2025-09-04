// Pattern Matching and URL Extraction Practice
//
// Learning Objectives:
// - Use regular expressions for pattern matching in text
// - Extract structured data from unstructured text
// - Handle different URL formats and edge cases
// - Practice with Rust's regex capabilities and string searching
// - Implement robust text parsing without external regex crate
//
// Run tests with: cargo test --lib string_manipulation::pattern_matcher

/// Extracts URLs from text using pattern matching.
///
/// This function should find URLs in the following formats:
/// - http://example.com
/// - https://www.example.com
/// - https://example.com/path/to/resource
/// - http://subdomain.example.com:8080/path?query=value
///
/// The function should:
/// - Return URLs in the order they appear in the text
/// - Handle URLs surrounded by whitespace, punctuation, or other text
/// - Not include trailing punctuation that's not part of the URL
/// - Be case-insensitive for the protocol part (http/https)
/// - Remove duplicate URLs from the result
///
/// Note: This implementation should not use external regex crates,
/// instead use string methods and iterator patterns for learning purposes.
///
/// # Arguments
/// * `text` - The input text to search for URLs
///
/// # Returns
/// A Vec<String> containing unique URLs found in the text, in order of first appearance
///
/// # Examples
/// ```
/// let text = "Visit https://example.com or http://test.org for more info.";
/// let urls = extract_urls(text);
/// assert_eq!(urls, vec!["https://example.com", "http://test.org"]);
/// ```
pub fn extract_urls(text: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_url_extraction() {
        let text = "Check out https://example.com for more information.";
        let urls = extract_urls(text);
        assert_eq!(urls, vec!["https://example.com"]);
    }

    #[test]
    fn test_multiple_urls() {
        let text = "Visit https://example.com and http://test.org and https://another-site.net";
        let urls = extract_urls(text);
        assert_eq!(
            urls,
            vec![
                "https://example.com",
                "http://test.org",
                "https://another-site.net"
            ]
        );
    }

    #[test]
    fn test_urls_with_paths_and_queries() {
        let text = "API endpoint: https://api.example.com/v1/users?limit=10&offset=0";
        let urls = extract_urls(text);
        assert_eq!(
            urls,
            vec!["https://api.example.com/v1/users?limit=10&offset=0"]
        );
    }

    #[test]
    fn test_urls_with_ports() {
        let text = "Development server at http://localhost:3000/app";
        let urls = extract_urls(text);
        assert_eq!(urls, vec!["http://localhost:3000/app"]);
    }

    #[test]
    fn test_case_insensitive_protocols() {
        let text = "Links: HTTP://EXAMPLE.COM and https://test.org";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        // Should normalize to lowercase protocols
        assert!(
            urls.contains(&"http://EXAMPLE.COM".to_string())
                || urls.contains(&"HTTP://EXAMPLE.COM".to_string())
        );
    }

    #[test]
    fn test_urls_with_punctuation() {
        let text = "Check (https://example.com), https://test.org. and https://third.com!";
        let urls = extract_urls(text);
        assert_eq!(
            urls,
            vec![
                "https://example.com",
                "https://test.org",
                "https://third.com"
            ]
        );
        // Should not include trailing punctuation
    }

    #[test]
    fn test_duplicate_removal() {
        let text = "Visit https://example.com and later https://example.com again";
        let urls = extract_urls(text);
        assert_eq!(urls, vec!["https://example.com"]);
    }

    #[test]
    fn test_no_urls() {
        let text = "This text has no URLs in it at all.";
        let urls = extract_urls(text);
        assert!(urls.is_empty());
    }

    #[test]
    fn test_complex_urls() {
        let text = r#"
            Production: https://api.example.com/v2/data?format=json&auth=token123
            Staging: https://staging-api.example.com:8443/v2/data
            Local: http://127.0.0.1:5000/debug
        "#;
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 3);
        assert!(
            urls.contains(&"https://api.example.com/v2/data?format=json&auth=token123".to_string())
        );
        assert!(urls.contains(&"https://staging-api.example.com:8443/v2/data".to_string()));
        assert!(urls.contains(&"http://127.0.0.1:5000/debug".to_string()));
    }

    #[test]
    fn test_malformed_urls() {
        let text = "Not URLs: htp://wrong.com, https:// incomplete, ftp://example.com";
        let urls = extract_urls(text);
        // Should only extract valid http/https URLs
        assert!(
            urls.is_empty()
                || !urls
                    .iter()
                    .any(|url| url.starts_with("htp://") || url.starts_with("ftp://"))
        );
    }
}
