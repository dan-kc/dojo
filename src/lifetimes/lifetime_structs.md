# Lifetime Structs - Solution

## Solution

```rust
pub struct KeyValuePair<'k, 'v> {
    key: &'k str,
    value: &'v str,
}

impl<'k, 'v> KeyValuePair<'k, 'v> {
    pub fn new(key: &'k str, value: &'v str) -> KeyValuePair<'k, 'v> {
        KeyValuePair { key, value }
    }
    
    pub fn key(&self) -> &'k str {
        self.key
    }
    
    pub fn value(&self) -> &'v str {
        self.value
    }
    
    pub fn format(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}

pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser { input, position: 0 }
    }
    
    pub fn remaining(&self) -> &'a str {
        &self.input[self.position..]
    }
    
    pub fn next_word(&mut self) -> Option<&'a str> {
        let remaining = self.remaining().trim_start();
        if remaining.is_empty() {
            return None;
        }
        
        let word_end = remaining
            .find(char::is_whitespace)
            .unwrap_or(remaining.len());
        
        let word_start = self.input.len() - remaining.len();
        let word = &self.input[word_start..word_start + word_end];
        
        self.position = word_start + word_end;
        Some(word)
    }
    
    pub fn peek_word(&self) -> Option<&'a str> {
        let remaining = self.remaining().trim_start();
        if remaining.is_empty() {
            return None;
        }
        
        let word_end = remaining
            .find(char::is_whitespace)
            .unwrap_or(remaining.len());
        
        Some(&remaining[..word_end])
    }
}

pub struct MultiRef<'a> {
    refs: Vec<&'a str>,
}

impl<'a> MultiRef<'a> {
    pub fn new() -> MultiRef<'a> {
        MultiRef { refs: Vec::new() }
    }
    
    pub fn add_ref(&mut self, r: &'a str) {
        self.refs.push(r);
    }
    
    pub fn get_refs(&self) -> &[&'a str] {
        &self.refs
    }
    
    pub fn find_longest(&self) -> Option<&'a str> {
        self.refs
            .iter()
            .max_by_key(|s| s.len())
            .copied()
    }
}

pub trait AsStrRef<'a> {
    fn as_str_ref(&self) -> &'a str;
}

pub struct NamedItem<'a> {
    name: &'a str,
}

impl<'a> NamedItem<'a> {
    pub fn new(name: &'a str) -> NamedItem<'a> {
        NamedItem { name }
    }
}

impl<'a> AsStrRef<'a> for NamedItem<'a> {
    fn as_str_ref(&self) -> &'a str {
        self.name
    }
}

pub struct MixedData<'a> {
    owned: String,
    borrowed: &'a str,
}

impl<'a> MixedData<'a> {
    pub fn new(owned: String, borrowed: &'a str) -> MixedData<'a> {
        MixedData { owned, borrowed }
    }
    
    pub fn owned_as_str(&self) -> &str {
        &self.owned
    }
    
    pub fn borrowed(&self) -> &'a str {
        self.borrowed
    }
    
    pub fn combine(&self) -> String {
        format!("{}{}", self.owned, self.borrowed)
    }
}
```

## Explanation

### Multiple Lifetime Parameters

**Independent Lifetimes:**
```rust
pub struct KeyValuePair<'k, 'v> {
    key: &'k str,
    value: &'v str,
}
```

**Key Benefits:**
- Key and value can have different lifetimes
- More flexible than using a single lifetime parameter
- Compiler can optimize based on actual lifetime relationships

**Implementation Considerations:**
- Each lifetime parameter must be declared in the struct and impl blocks
- Methods can return references with specific lifetimes from the original data

### Self-Referential Patterns

**Parser State Management:**
```rust
pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}
```

**Advanced Techniques:**
- All returned references have lifetime `'a` tied to original input
- Position tracking allows stateful parsing without copying data
- `peek_word()` vs `next_word()` demonstrates immutable vs mutable methods with same lifetime guarantees

**String Slice Mathematics:**
- Calculate absolute positions in original string
- Maintain reference validity through position arithmetic
- Handle whitespace and edge cases correctly

### Collection Lifetime Management

**Storing Multiple References:**
```rust
pub struct MultiRef<'a> {
    refs: Vec<&'a str>,
}
```

**Lifetime Constraints:**
- All stored references must live at least as long as `'a`
- Adding references requires they satisfy the lifetime bound
- Compiler ensures no dangling references in the collection

### Trait Lifetimes

**Associated Lifetimes in Traits:**
```rust
pub trait AsStrRef<'a> {
    fn as_str_ref(&self) -> &'a str;
}
```

**Implementation Strategy:**
- Lifetime parameter on trait allows return references to implementor's data
- Trait implementations must maintain lifetime relationships
- Enables generic programming over types that can provide string references

### Mixed Ownership Patterns

**Combining Owned and Borrowed Data:**
```rust
pub struct MixedData<'a> {
    owned: String,
    borrowed: &'a str,
}
```

**Design Patterns:**
- Owned data doesn't need lifetime annotations
- Borrowed data requires lifetime parameter
- Methods can return references with different lifetime properties:
  - `owned_as_str()` returns `&str` (tied to self)
  - `borrowed()` returns `&'a str` (tied to original source)

### Advanced Lifetime Concepts

**Lifetime Variance:**
- Covariance: `&'long str` can be used where `&'short str` is expected
- Enables flexibility in lifetime relationships
- Compiler automatically handles lifetime subtyping

**Lifetime Bounds:**
- `'a: 'b` means lifetime `'a` outlives lifetime `'b`
- Useful for constraining relationships between multiple lifetimes
- Enables complex data structures with guaranteed safety

**Common Patterns:**
1. **Parser Pattern**: Single lifetime for input data, all outputs tied to input
2. **Multiple References Pattern**: Different lifetimes for independent data sources
3. **Collection Pattern**: Single lifetime constraint for all stored references
4. **Mixed Ownership Pattern**: Combine owned and borrowed data safely

**Error Prevention:**
- Compiler prevents use-after-free errors
- Ensures all references remain valid for their declared lifetimes
- Provides clear error messages when lifetime relationships are violated