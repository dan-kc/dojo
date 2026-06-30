````rust
```rust
pub fn spell_check(
    text: &str,
    dictionary: &std::collections::HashSet<String>,
) -> std::collections::HashSet<String> {
    let mut res = std::collections::HashSet::new();
    if text.is_empty() {
        return res;
    };

    for word in text.split_ascii_whitespace() {
        if !dictionary.contains(word) {
            res.insert(word.to_string());
        }
    }

    return res;
}
````
