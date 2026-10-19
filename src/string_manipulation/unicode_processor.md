# string_manipulation/unicode_processor

Reference implementation. Read after attempting the exercise.

```rust
pub fn normalize_text(text: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    let cleaned: String = text
        .chars()
        .filter_map(|c| match c {
            '\u{200B}'..='\u{200F}'
            | '\u{FEFF}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2066}'..='\u{2069}' => None,
            '\u{2018}' | '\u{2019}' => Some('\''),
            '\u{201C}' | '\u{201D}' => Some('"'),
            '\u{2013}' | '\u{2014}' => Some('-'),
            c if c.is_control() && !c.is_whitespace() => None,
            c => Some(c),
        })
        .collect();
    cleaned
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .nfc()
        .collect()
}
```
