# Find Longest String

## Solution

```rust
pub fn find_longest_string(strings: &[&str]) -> Option<&str> {
    strings.iter()
        .max_by_key(|s| s.len())
        .copied()
}
```

## Explanation

This solution demonstrates lifetime relationships when returning references from collections:

1. **Iterator Over References**: `iter()` creates an iterator over `&&str` (references to references)
2. **Max By Key**: Finds the element with maximum length
3. **Copied**: Converts `&&str` to `&str` by dereferencing

Key concepts:
- **Lifetime Propagation**: The returned `&str` has the same lifetime as the strings in the input slice
- **Double References**: Working with `&[&str]` means dealing with `&&str` in iterators
- **Option Handling**: `max_by_key` returns `None` for empty iterators automatically
- **Stable Selection**: Returns the first occurrence when multiple strings have the same max length

Alternative implementation:
```rust
pub fn find_longest_string(strings: &[&str]) -> Option<&str> {
    if strings.is_empty() {
        return None;
    }
    
    let mut longest = strings[0];
    for &s in &strings[1..] {
        if s.len() > longest.len() {
            longest = s;
        }
    }
    Some(longest)
}
```

The iterator approach is more idiomatic and handles the empty case automatically without explicit checks.