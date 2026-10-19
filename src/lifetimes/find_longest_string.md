# lifetimes/find_longest_string

Reference implementation. Read after attempting the exercise.

```rust
pub fn find_longest_string<'a>(strings: &[&'a str]) -> Option<&'a str> {
    strings
        .iter()
        .copied()
        .reduce(|a, b| if b.len() > a.len() { b } else { a })
}
```
