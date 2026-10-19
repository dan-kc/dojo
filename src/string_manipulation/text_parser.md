# string_manipulation/text_parser

Reference implementation. Read after attempting the exercise.

```rust
pub fn parse_key_value_pairs(
    input: &str,
) -> Result<std::collections::HashMap<String, String>, String> {
    let mut out = std::collections::HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line
            .split_once('=')
            .ok_or_else(|| format!("Line {}: missing '='", i + 1))?;
        let key = key.trim();
        if key.is_empty() {
            return Err(format!("Line {}: empty key", i + 1));
        }
        out.insert(key.to_owned(), value.trim().to_owned());
    }
    Ok(out)
}
```
