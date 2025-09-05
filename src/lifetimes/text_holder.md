# Text Holder

## Solution

```rust
pub struct TextHolder<'a> {
    text: &'a str,
}

impl<'a> TextHolder<'a> {
    pub fn new(text: &'a str) -> TextHolder<'a> {
        TextHolder { text }
    }
    
    pub fn get_text(&self) -> &'a str {
        self.text
    }
    
    pub fn get_prefix(&self, n: usize) -> &'a str {
        let end = n.min(self.text.len());
        &self.text[..end]
    }
}
```

## Explanation

This solution demonstrates lifetime parameters in structs:

1. **Struct Lifetime**: `TextHolder<'a>` declares that the struct contains references with lifetime `'a`
2. **Field Lifetime**: The `text` field has the same lifetime as the struct parameter
3. **Implementation Block**: `impl<'a>` introduces the lifetime for the implementation
4. **Method Returns**: Methods can return references with the same lifetime as the stored data

Key concepts:
- **Lifetime Parameter**: `'a` represents the lifetime of the borrowed text
- **Lifetime Propagation**: The struct cannot outlive the text it references
- **Safe Slicing**: `get_prefix` uses `min` to avoid panics from out-of-bounds slicing
- **Elision in Methods**: `&self` automatically gets its own lifetime, separate from `'a`

Important notes:
- The struct can only live as long as the text it references
- Methods returning `&'a str` return references tied to the original text, not to `self`
- This pattern is common for zero-copy string processing and parsers

Common mistakes to avoid:
```rust
// Wrong: trying to return reference with lifetime of self
pub fn get_text(&self) -> &str {  // This actually works due to elision
    self.text
}

// Wrong: forgetting lifetime parameter on impl
impl TextHolder<'a> {  // Missing <'a> after impl
    // ...
}
```