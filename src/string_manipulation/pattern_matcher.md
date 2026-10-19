# string_manipulation/pattern_matcher

Reference implementation. Read after attempting the exercise.

```rust
pub fn extract_urls(text: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for token in text.split_whitespace() {
        let token = token
            .trim_start_matches(['(', '[', '{', '<', '\'', '"'])
            .trim_end_matches([')', ']', '}', '>', '\'', '"', ',', '.', '!', '?', ';']);
        let prefix = if token
            .get(..7)
            .is_some_and(|s| s.eq_ignore_ascii_case("http://"))
        {
            7
        } else if token
            .get(..8)
            .is_some_and(|s| s.eq_ignore_ascii_case("https://"))
        {
            8
        } else {
            continue;
        };
        let rest = &token[prefix..];
        if rest.split(['/', '?', '#']).next().unwrap_or("").is_empty() {
            continue;
        }
        let url = format!("{}{}", token[..prefix].to_ascii_lowercase(), rest);
        if seen.insert(url.clone()) {
            out.push(url);
        }
    }
    out
}
```
