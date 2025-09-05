# Build URL Query String

## Solution

```rust
use std::collections::HashMap;

pub fn build_query_string(params: &HashMap<String, Option<String>>) -> String {
    let mut pairs: Vec<_> = params
        .iter()
        .filter_map(|(key, value)| {
            // Skip None values, but include empty string values
            value.as_ref().map(|v| (key, v))
        })
        .collect();
    
    // Sort alphabetically by key for consistent output
    pairs.sort_by(|a, b| a.0.cmp(b.0));
    
    pairs
        .into_iter()
        .map(|(key, value)| {
            format!("{}={}", url_encode(key), url_encode(value))
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn url_encode(input: &str) -> String {
    let mut result = String::with_capacity(input.len() * 3);
    for ch in input.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(ch);
            }
            ' ' => result.push_str("%20"),
            _ => {
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

This solution demonstrates **efficient string building** with **URL encoding** and **parameter handling**:

### Key Concepts Demonstrated:

1. **HashMap Processing with Filtering**:
   - `filter_map()` combines filtering and transformation in one step
   - Skips `None` values but preserves empty strings (`Some("")`)
   - Maintains type safety throughout the transformation pipeline

2. **Sorting for Consistency**:
   - Sorts parameters alphabetically by key for predictable output
   - Essential for testing, caching, and API consistency
   - `sort_by()` with key comparison ensures stable ordering

3. **URL Encoding Rules**:
   - Safe characters (alphanumeric, `-`, `_`, `.`, `~`) pass through unchanged
   - Space becomes `%20` (not `+` in this implementation)
   - All other characters encoded as `%XX` using uppercase hexadecimal
   - Handles UTF-8 multibyte characters correctly

4. **Efficient String Building**:
   - Pre-allocates capacity for encoding to reduce reallocations
   - Uses `join()` for final concatenation instead of repeated `+` operations
   - Formats key-value pairs before joining for better performance

### URL Encoding Details:

```rust
// Safe characters (RFC 3986 unreserved characters):
'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~'

// Special encoding cases:
' '  → "%20"    // Space
'&'  → "%26"    // Ampersand (separates parameters)
'='  → "%3D"    // Equals (separates key from value)
'%'  → "%25"    // Percent (escape character itself)
'+'  → "%2B"    // Plus sign
```

### UTF-8 Multibyte Handling:

```rust
// Example: "café" encoding
'c' → 'c'           // Safe character
'a' → 'a'           // Safe character  
'f' → 'f'           // Safe character
'é' → "%C3%A9"      // UTF-8 bytes: [0xC3, 0xA9]
```

The solution correctly handles Unicode by:
1. Converting each char to its UTF-8 byte representation
2. Encoding each byte as `%XX` hexadecimal
3. Maintaining proper UTF-8 encoding throughout

### Parameter Processing Pipeline:

```rust
HashMap<String, Option<String>>     // Input parameters
  .iter()                          → Iterator<(&String, &Option<String>)>
  .filter_map(|(k, v)| ...)        → Iterator<(&String, &String)> 
  .collect()                       → Vec<(&String, &String)>
  .sort_by(...)                    → Sorted Vec<(&String, &String)>
  .into_iter()                     → Iterator<(&String, &String)>
  .map(|(k, v)| format!(...))      → Iterator<String>
  .collect::<Vec<_>>()             → Vec<String>
  .join("&")                       → String
```

### Memory Efficiency Strategies:

1. **Capacity Pre-allocation**: `String::with_capacity(input.len() * 3)` assumes worst-case encoding
2. **Single Allocation for Join**: `join()` calculates total size before allocating
3. **Iterator Chains**: Minimize intermediate allocations through lazy evaluation
4. **In-place Sorting**: Sorts references instead of cloning data

### Edge Cases Handled:

```rust
// Empty parameters
HashMap::new() → ""

// None values (excluded)
{"key": None} → ""

// Empty string values (included)  
{"key": Some("")} → "key="

// Special characters in keys and values
{"ke&y": Some("val=ue")} → "ke%26y=val%3Due"

// Unicode characters
{"café": Some("naïve")} → "caf%C3%A9=na%C3%AFve"
```

### Alternative Implementations:

```rust
// Using format! for each parameter (less efficient):
pub fn build_query_string_v2(params: &HashMap<String, Option<String>>) -> String {
    let mut result = String::new();
    let mut sorted_keys: Vec<_> = params.keys().collect();
    sorted_keys.sort();
    
    for (i, key) in sorted_keys.iter().enumerate() {
        if let Some(Some(value)) = params.get(*key) {
            if i > 0 { result.push('&'); }
            result.push_str(&format!("{}={}", url_encode(key), url_encode(value)));
        }
    }
    result
}

// Using BTreeMap for automatic sorting:
pub fn build_query_string_v3(params: &std::collections::BTreeMap<String, Option<String>>) -> String {
    params
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| format!("{}={}", url_encode(k), url_encode(val))))
        .collect::<Vec<_>>()
        .join("&")
}
```

### Performance Considerations:

- **Sorting Overhead**: O(n log n) for alphabetical ordering
- **URL Encoding**: O(m) where m is total character count
- **String Joining**: Single allocation when using `join()`
- **Memory Usage**: 3x capacity pre-allocation for worst-case encoding

### Real-World Applications:

- **HTTP Clients**: Building query strings for API requests
- **Form Handling**: Processing web form submissions
- **URL Generation**: Creating URLs with parameters for links
- **Cache Keys**: Generating consistent cache keys from parameters
- **Logging**: Formatting request parameters for logging

### Security Considerations:

1. **Proper Encoding**: Prevents injection attacks through URL parameters
2. **Consistent Output**: Sorted parameters prevent cache poisoning
3. **Character Safety**: All special characters properly escaped
4. **UTF-8 Handling**: International characters handled correctly

This solution demonstrates how to build robust, efficient string processing functions that handle encoding, sorting, and edge cases while maintaining good performance characteristics and security properties.