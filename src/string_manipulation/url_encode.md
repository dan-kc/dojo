# string_manipulation/url_encode

Reference implementation. Read after attempting the exercise.

```rust
pub fn url_encode(input: &str) -> String {
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
