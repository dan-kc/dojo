# string_manipulation/build_query_string

Reference implementation. Read after attempting the exercise.

```rust
pub fn build_query_string(params: &std::collections::HashMap<String, Option<String>>) -> String {
    let mut entries: Vec<_> = params
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|v| (k, v)))
        .collect();
    entries.sort_unstable_by(|a, b| a.0.cmp(b.0));
    entries
        .into_iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<_>>()
        .join("&")
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
```
