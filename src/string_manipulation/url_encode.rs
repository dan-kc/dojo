// URL Encoding Practice
//
// Learning Objectives:
// - Understand URL/percent encoding rules
// - Work with character classification and transformation
// - Practice efficient string building
// - Handle UTF-8 and special characters correctly
//
// cargo test --lib string_manipulation::url_encode

/// Implement URL/percent encoding for a given string.
///
/// This function should properly encode special characters according to URL encoding rules.
/// For learning purposes, implement this without using external crates.
///
/// URL encoding rules:
/// - Alphanumeric characters (A-Z, a-z, 0-9) remain unchanged
/// - Hyphen (-), underscore (_), period (.), and tilde (~) are safe and remain unchanged
/// - Space becomes '%20'
/// - All other characters are percent-encoded using their UTF-8 byte representation
/// - Each byte is encoded as %XX where XX is the uppercase hexadecimal value
///
/// Examples:
/// - "hello world" -> "hello%20world"
/// - "user@example.com" -> "user%40example.com"
/// - "100%" -> "100%25"
/// - "café" -> "caf%C3%A9" (é is encoded as UTF-8 bytes C3 A9)
///
/// # Arguments
/// * `input` - The string to URL-encode
///
/// # Returns
/// The URL-encoded string
pub fn url_encode(input: &str) -> String {
    todo!("Implement URL encoding with proper character handling")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_encoding() {
        assert_eq!(url_encode("hello world"), "hello%20world");
        assert_eq!(url_encode("test string"), "test%20string");
    }

    #[test]
    fn test_safe_characters() {
        // These characters should not be encoded
        assert_eq!(url_encode("safe-chars_123.~"), "safe-chars_123.~");
        assert_eq!(url_encode("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~"), 
                   "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_.~");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(url_encode("user@example.com"), "user%40example.com");
        assert_eq!(url_encode("name=value&other=data"), "name%3Dvalue%26other%3Ddata");
        assert_eq!(url_encode("http://example.com?foo=bar"), "http%3A%2F%2Fexample.com%3Ffoo%3Dbar");
    }

    #[test]
    fn test_percent_encoding() {
        assert_eq!(url_encode("100%"), "100%25");
        assert_eq!(url_encode("%%"), "%25%25");
    }

    #[test]
    fn test_symbols() {
        assert_eq!(url_encode("!@#$%^&*()"), "%21%40%23%24%25%5E%26%2A%28%29");
        assert_eq!(url_encode("[]{}"), "%5B%5D%7B%7D");
        assert_eq!(url_encode("|\\"), "%7C%5C");
        assert_eq!(url_encode("<>"), "%3C%3E");
    }

    #[test]
    fn test_quotes() {
        assert_eq!(url_encode("'single'"), "%27single%27");
        assert_eq!(url_encode("\"double\""), "%22double%22");
    }

    #[test]
    fn test_unicode_characters() {
        // UTF-8 encoding of unicode characters
        assert_eq!(url_encode("café"), "caf%C3%A9");
        assert_eq!(url_encode("naïve"), "na%C3%AFve");
        assert_eq!(url_encode("日本語"), "%E6%97%A5%E6%9C%AC%E8%AA%9E");
        assert_eq!(url_encode("😀"), "%F0%9F%98%80");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(url_encode(""), "");
    }

    #[test]
    fn test_whitespace_variations() {
        assert_eq!(url_encode(" "), "%20");
        assert_eq!(url_encode("  "), "%20%20");
        assert_eq!(url_encode("\t"), "%09");
        assert_eq!(url_encode("\n"), "%0A");
        assert_eq!(url_encode("\r"), "%0D");
    }

    #[test]
    fn test_mixed_content() {
        assert_eq!(url_encode("Hello World! 123 #test"), "Hello%20World%21%20123%20%23test");
        assert_eq!(url_encode("param=value with spaces&another=test"), 
                   "param%3Dvalue%20with%20spaces%26another%3Dtest");
    }

    #[test]
    fn test_plus_sign() {
        // Plus sign should be encoded (not used as space replacement in this implementation)
        assert_eq!(url_encode("1+1=2"), "1%2B1%3D2");
    }

    #[test]
    fn test_slash_encoding() {
        assert_eq!(url_encode("/path/to/file"), "%2Fpath%2Fto%2Ffile");
        assert_eq!(url_encode("\\backslash"), "%5Cbackslash");
    }

    #[test]
    fn test_colon_semicolon() {
        assert_eq!(url_encode("time:12:30"), "time%3A12%3A30");
        assert_eq!(url_encode("item1;item2"), "item1%3Bitem2");
    }

    #[test]
    fn test_question_mark_hash() {
        assert_eq!(url_encode("query?param"), "query%3Fparam");
        assert_eq!(url_encode("anchor#section"), "anchor%23section");
    }
}