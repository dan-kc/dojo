# Split First Word

## Solution

```rust
pub fn split_first_word(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(pos) => (&s[..pos], &s[pos + 1..]),
        None => (s, ""),
    }
}
```

## Explanation

This solution demonstrates lifetime elision and string slice manipulation:

1. **Lifetime Elision**: The compiler automatically infers that both returned references have the same lifetime as the input
2. **Pattern Matching**: Uses `match` to handle both cases elegantly
3. **String Slicing**: Creates subslices of the original string without allocation

Key concepts:
- **Elision Rules**: When there's one input lifetime, it's automatically assigned to all output lifetimes
- **Zero-Copy**: Returns views into the original string, no new allocations
- **UTF-8 Safety**: `find(' ')` returns a byte position that's safe for slicing
- **Ownership**: The returned slices are valid as long as the input string is valid

The lifetime elision rules mean this signature:
```rust
fn split_first_word(s: &str) -> (&str, &str)
```

Is equivalent to:
```rust
fn split_first_word<'a>(s: &'a str) -> (&'a str, &'a str)
```

This pattern is common in string processing where you want to return multiple views into the same data without copying.