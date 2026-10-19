# lifetimes/lifetime_structs

Reference implementation. Read after attempting the exercise.

```rust
pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }
    pub fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }
    pub fn peek_word(&self) -> Option<&'a str> {
        self.remaining().split_whitespace().next()
    }
    pub fn next_word(&mut self) -> Option<&'a str> {
        let rest = self.remaining();
        let trimmed = rest.trim_start();
        if trimmed.is_empty() {
            self.position = self.input.len();
            return None;
        }
        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        self.position = self.input.len() - trimmed.len() + end;
        Some(&trimmed[..end])
    }
}
```
