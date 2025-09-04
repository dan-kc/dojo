# Lifetime Basics - Solution

## Solution

```rust
pub fn longer_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

pub fn split_first_word(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(pos) => (&s[..pos], &s[pos + 1..]),
        None => (s, ""),
    }
}

pub fn find_longest_string(strings: &[&str]) -> Option<&str> {
    strings
        .iter()
        .max_by_key(|s| s.len())
        .copied()
}

pub fn repeat_longer_owned(s1: &str, s2: &str) -> String {
    let longer = if s1.len() >= s2.len() { s1 } else { s2 };
    format!("{}{}", longer, longer)
}

pub struct TextHolder<'a> {
    text: &'a str,
}

impl<'a> TextHolder<'a> {
    pub fn new(text: &'a str) -> TextHolder<'a> {
        TextHolder { text }
    }
    
    pub fn get_text(&self) -> &str {
        self.text
    }
    
    pub fn get_prefix(&self, n: usize) -> &str {
        if n >= self.text.len() {
            self.text
        } else {
            &self.text[..n]
        }
    }
}

pub fn after_char(s: &str, c: char) -> &str {
    match s.find(c) {
        Some(pos) => &s[pos + 1..],
        None => s,
    }
}
```

## Explanation

### Lifetime Annotation Fundamentals

**Basic Syntax:**
- `'a` is a lifetime parameter (can be any name, but `'a`, `'b`, etc. are conventional)
- `&'a str` means "a reference with lifetime 'a to a str"
- Function signatures must declare lifetime parameters: `fn foo<'a>(...)`

**Lifetime Relationships:**
```rust
pub fn longer_string<'a>(s1: &'a str, s2: &'a str) -> &'a str
```
This signature means: "Both inputs live for at least lifetime 'a, and the output lives for exactly lifetime 'a"

### Lifetime Elision Rules

**When You Don't Need Annotations:**
```rust
pub fn split_first_word(s: &str) -> (&str, &str)
pub fn after_char(s: &str, c: char) -> &str
```

The compiler applies these elision rules:
1. Each input reference gets its own lifetime
2. If there's exactly one input lifetime, it's assigned to all outputs
3. If there's a `&self` parameter, its lifetime is assigned to all outputs

### Struct Lifetimes

**Lifetime Parameters in Structs:**
```rust
pub struct TextHolder<'a> {
    text: &'a str,
}
```

**Key Points:**
- Struct lifetime parameter must be declared: `TextHolder<'a>`
- All impl blocks must include the lifetime: `impl<'a> TextHolder<'a>`
- Methods can return references with the same lifetime as struct fields

**Lifetime Relationships in Methods:**
```rust
pub fn get_prefix(&self, n: usize) -> &str
```
This uses elision - the returned reference has the same lifetime as `&self`.

### Common Patterns and Solutions

**Multiple Input Lifetimes:**
When returning a reference that could come from either input, both inputs need the same lifetime annotation.

**Owned Return Types:**
```rust
pub fn repeat_longer_owned(s1: &str, s2: &str) -> String
```
No lifetime annotations needed because we're returning owned data.

**Working with Collections:**
```rust
pub fn find_longest_string(strings: &[&str]) -> Option<&str>
```
The returned reference has the same lifetime as the slice elements (elision rules apply).

### Compilation and Borrowing Rules

**Lifetime Validity:**
- The returned reference cannot outlive the shortest input lifetime
- Rust prevents dangling references at compile time
- Struct instances cannot outlive their borrowed data

**Common Lifetime Errors:**
1. **Returning references to local variables** - solved by returning owned types
2. **Mismatched lifetimes in function signatures** - solved by finding the common lifetime
3. **Struct lifetime issues** - solved by ensuring data outlives struct instances

**Best Practices:**
1. Start without lifetime annotations and let the compiler guide you
2. Use owned types (`String`, `Vec<T>`) when lifetime management becomes complex
3. Prefer borrowing when possible, but don't fight the borrow checker unnecessarily