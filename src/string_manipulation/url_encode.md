# URL Encoding Implementation

## Solution

```rust
pub fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    
    for ch in input.chars() {
        match ch {
            // Safe characters that don't need encoding
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(ch);
            }
            // Space character (common case)
            ' ' => result.push_str("%20"),
            // All other characters need percent encoding
            _ => {
                // Convert character to UTF-8 bytes and encode each byte
                for byte in ch.to_string().bytes() {
                    result.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    
    result
}
```

## Explanation

This solution implements **URL/percent encoding** following RFC 3986 standards:

### Key Concepts Demonstrated:

1. **Character Classification**:
   - **Unreserved characters**: `A-Z`, `a-z`, `0-9`, `-`, `_`, `.`, `~` (safe, no encoding needed)
   - **Space character**: Special case encoded as `%20`
   - **Reserved/Special characters**: Everything else requires percent encoding

2. **UTF-8 Byte Encoding**:
   - Characters are converted to UTF-8 byte sequences
   - Each byte is encoded as `%XX` where `XX` is uppercase hexadecimal
   - Handles Unicode characters correctly by encoding their UTF-8 representation

3. **Efficient String Building**:
   - Pre-allocates capacity assuming worst-case (every char needs 3 bytes: `%XX`)
   - Uses `push()` for single characters and `push_str()` for encoded sequences
   - Minimizes reallocations for better performance

4. **Format Specifier**: 
   - `%{:02X}` formats bytes as uppercase hexadecimal with zero-padding
   - `02` ensures two digits (e.g., `%0A` not `%A`)
   - `X` produces uppercase hex (required by URL encoding standards)

### URL Encoding Rules (RFC 3986):

```rust
// Unreserved characters (never encoded):
ALPHA   = A-Z / a-z
DIGIT   = 0-9  
HYPHEN  = -
PERIOD  = .
UNDERSCORE = _
TILDE   = ~

// Reserved characters (context-dependent encoding):
gen-delims    = : / ? # [ ] @
sub-delims    = ! $ & ' ( ) * + , ; =

// Percent encoding format:
pct-encoded   = "%" HEXDIG HEXDIG
```

### UTF-8 Multibyte Character Handling:

```rust
// Example encodings:
"café" → "caf%C3%A9"
// 'c' → 'c' (safe)
// 'a' → 'a' (safe)  
// 'f' → 'f' (safe)
// 'é' → UTF-8: [0xC3, 0xA9] → "%C3%A9"

"日本語" → "%E6%97%A5%E6%9C%AC%E8%AA%9E"
// '日' → UTF-8: [0xE6, 0x97, 0xA5] → "%E6%97%A5"
// '本' → UTF-8: [0xE6, 0x9C, 0xAC] → "%E6%9C%AC"  
// '語' → UTF-8: [0xE8, 0xAA, 0x9E] → "%E8%AA%9E"

"😀" → "%F0%9F%98%80"
// '😀' → UTF-8: [0xF0, 0x9F, 0x98, 0x80] → "%F0%9F%98%80"
```

### Performance Optimizations:

1. **Capacity Pre-allocation**:
   ```rust
   String::with_capacity(input.len() * 3)
   // Assumes worst case: every character becomes %XX
   // Prevents multiple reallocations during building
   ```

2. **Character-by-Character Processing**:
   ```rust
   for ch in input.chars()  // Handles Unicode correctly
   // vs input.bytes()     // Would break multibyte characters
   ```

3. **Efficient Byte Encoding**:
   ```rust
   ch.to_string().bytes()  // Convert to UTF-8 bytes
   // Alternative: ch.encode_utf8(&mut buffer).bytes()
   ```

### Memory Layout Analysis:

```
Input:    "hello world"
          [UTF-8 bytes in memory]
          
Processing:
'h' → 'h'        (1 byte → 1 char)
'e' → 'e'        (1 byte → 1 char)  
'l' → 'l'        (1 byte → 1 char)
'l' → 'l'        (1 byte → 1 char)
'o' → 'o'        (1 byte → 1 char)
' ' → "%20"      (1 byte → 3 chars)
'w' → 'w'        (1 byte → 1 char)
...

Output:   "hello%20world"
          [12 bytes total vs 11 input bytes]
```

### Edge Cases Handled:

```rust
// Empty string
"" → ""

// Only safe characters  
"hello123" → "hello123"

// Only unsafe characters
"!@#$%" → "%21%40%23%24%25"

// Mixed Unicode and ASCII
"Hello 世界!" → "Hello%20%E4%B8%96%E7%95%8C%21"

// Whitespace variations
" \t\n\r" → "%20%09%0A%0D"
```

### Alternative Implementations:

```rust
// Using a lookup table for common characters:
const ENCODE_TABLE: &[&str; 256] = [...]; // Pre-computed encodings

pub fn url_encode_table(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    
    for byte in input.bytes() {
        result.push_str(ENCODE_TABLE[byte as usize]);
    }
    
    result
}

// Using iterators and collect:
pub fn url_encode_functional(input: &str) -> String {
    input.chars()
        .flat_map(|ch| match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                vec![ch.to_string()]
            }
            ' ' => vec!["%20".to_string()],
            _ => ch.to_string().bytes()
                   .map(|b| format!("%{:02X}", b))
                   .collect(),
        })
        .collect::<Vec<_>>()
        .join("")
}
```

### Security Implications:

1. **Injection Prevention**: Properly encodes special characters that could be used in attacks
2. **Data Integrity**: Ensures URL parameters are transmitted correctly  
3. **Standard Compliance**: Follows RFC 3986 for interoperability
4. **Unicode Safety**: Handles international characters without data loss

### Real-World Usage:

- **Web Forms**: Encoding form data for HTTP requests
- **URL Building**: Creating URLs with user-provided parameters  
- **API Clients**: Encoding query parameters and path components
- **Data Serialization**: Safe transmission of text data in URLs
- **Security**: Preventing URL-based injection attacks

### Performance Characteristics:

- **Time Complexity**: O(n×m) where n is string length, m is avg bytes per character
- **Space Complexity**: O(n×k) where k is encoding expansion factor (≤3)
- **Memory Allocation**: Single allocation with good capacity estimation
- **UTF-8 Handling**: Correct but not optimized for large Unicode strings

This implementation provides a robust, standards-compliant URL encoding function that handles Unicode correctly while maintaining good performance for typical use cases.