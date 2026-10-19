# lifetimes/split_first_word

Reference implementation. Read after attempting the exercise.

```rust
pub fn split_first_word(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(pos) => (&s[..pos], &s[pos + 1..]),
        None => (s, ""),
    }
}
```
