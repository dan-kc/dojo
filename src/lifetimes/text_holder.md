# lifetimes/text_holder

Reference implementation. Read after attempting the exercise.

```rust
pub struct TextHolder<'a> {
    text: &'a str,
}
impl<'a> TextHolder<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }
    pub fn get_text(&self) -> &'a str {
        self.text
    }
    pub fn get_prefix(&self, n: usize) -> &'a str {
        let end = self
            .text
            .char_indices()
            .nth(n)
            .map_or(self.text.len(), |(i, _)| i);
        &self.text[..end]
    }
}
```
