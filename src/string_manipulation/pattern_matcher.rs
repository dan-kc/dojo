// cargo test --lib string_manipulation::pattern_matcher::

//! Extract HTTP(S) links from whitespace-separated tokens. Trim leading (, [, {, <, single/double
//! quotes; trim trailing ), ], }, >, quotes, comma, dot, exclamation, question mark, semicolon.
//! Recognize http:// or https:// case-insensitively at the start of the remaining token and
//! lowercase only that scheme. Require a nonempty authority before the first /, ? or #.
//! Deduplicate the resulting strings while preserving first-seen order. Preserve host/path case.
//! This is a precisely scoped text heuristic, not a URL validator; do not use regex.

pub fn extract_urls(text: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokens_case_and_order() {
        assert_eq!(extract_urls("雪 (HTTPS://Example.com/a?q=1), http://localhost:3000/x! HTTPS://Example.com/a?q=1 https://example.com/a?q=1"),["https://Example.com/a?q=1","http://localhost:3000/x","https://example.com/a?q=1"]);
    }
    #[test]
    fn malformed_and_empty() {
        assert!(extract_urls("http:// https:///path htp://x www.x.test words").is_empty());
        assert!(extract_urls("").is_empty());
    }
    #[test]
    fn surrounding_and_fragment() {
        assert_eq!(
            extract_urls("[https://a/x#part] \"http://b\" <https://c>"),
            ["https://a/x#part", "http://b", "https://c"]
        );
    }
}
