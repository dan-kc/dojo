# After Char

## Solution

```rust
pub fn after_char(s: &str, c: char) -> &str {
    match s.find(c) {
        Some(pos) => {
            let char_end = pos + c.len_utf8();
            &s[char_end..]
        }
        None => s,
    }
}
```

## Explanation

This solution demonstrates lifetime elision and safe Unicode handling:

1. **Character Finding**: `find(c)` returns the byte position of the first occurrence
2. **Unicode Safety**: `c.len_utf8()` gets the correct byte length of the character
3. **Slice Creation**: Creates a slice starting after the found character
4. **Fallback**: Returns the original string if character not found

Key concepts:
- **Lifetime Elision**: No explicit lifetime annotations needed - the compiler infers the output lifetime matches the input
- **Unicode Awareness**: Properly handles multi-byte UTF-8 characters
- **Safe Slicing**: Using `find()` ensures the slice position is at a character boundary
- **Zero-Copy**: Returns a view into the original string without allocation

Why lifetime elision works here:
- Single input reference parameter
- Output is a reference to the same data
- Compiler automatically assigns input lifetime to output

Handling Unicode:
- `find()` returns byte positions, which are always safe for slicing
- `char.len_utf8()` gives the correct number of bytes to skip
- This approach works correctly with any Unicode character

Alternative (less safe) approach:
```rust
// Don't do this - assumes ASCII
pub fn after_char_ascii_only(s: &str, c: char) -> &str {
    match s.find(c) {
        Some(pos) => &s[pos + 1..], // Wrong for multi-byte chars
        None => s,
    }
}
```