# collections/palindrome_checker

Reference implementation. Read after attempting the exercise.

```rust
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    chars.iter().eq(chars.iter().rev())
}
```
