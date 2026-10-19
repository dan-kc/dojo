# lifetimes/after_char

Reference implementation. Read after attempting the exercise.

```rust
pub fn after_char(s: &str, c: char) -> &str {
    match s.find(c) {
        Some(pos) => {
            let char_end = pos + c.len_utf8();
            &s[char_end..]
        }
        None => s,
    }
}
```
