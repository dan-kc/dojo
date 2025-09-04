# Pattern Matching and URL Extraction - Solution

## Complete Implementation

```rust
pub fn extract_urls(text: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut seen = std::collections::HashSet::new();

    // Split text into words to find potential URLs
    let words = text.split_whitespace();

    for word in words {
        // Check if word starts with http:// or https:// (case insensitive)
        let lower_word = word.to_lowercase();
        if lower_word.starts_with("http://") || lower_word.starts_with("https://") {
            // Clean up trailing punctuation that's not part of the URL
            let cleaned_url = clean_trailing_punctuation(word);

            // Validate that it looks like a proper URL
            if is_valid_url(&cleaned_url) {
                // Keep original case for the URL (except protocol normalization)
                let normalized_url = normalize_protocol(&cleaned_url);

                // Only add if we haven't seen it before
                if seen.insert(normalized_url.clone()) {
                    urls.push(normalized_url);
                }
            }
        }
    }

    urls
}

fn clean_trailing_punctuation(url: &str) -> String {
    let mut result = url.to_string();

    // Remove trailing punctuation that's typically not part of URLs
    while let Some(last_char) = result.chars().last() {
        match last_char {
            '.' | ',' | '!' | '?' | ';' | ')' | ']' | '}' | '"' | '\'' => {
                result.pop();
            }
            _ => break,
        }
    }

    result
}

fn normalize_protocol(url: &str) -> String {
    if url.to_lowercase().starts_with("http://") {
        format!("http://{}", &url[7..])
    } else if url.to_lowercase().starts_with("https://") {
        format!("https://{}", &url[8..])
    } else {
        url.to_string()
    }
}

fn is_valid_url(url: &str) -> bool {
    let lower_url = url.to_lowercase();

    // Must start with http:// or https://
    if !lower_url.starts_with("http://") && !lower_url.starts_with("https://") {
        return false;
    }

    // Extract the part after the protocol
    let after_protocol = if lower_url.starts_with("https://") {
        &url[8..]
    } else {
        &url[7..]
    };

    // Must have something after the protocol
    if after_protocol.is_empty() {
        return false;
    }

    // Split by '/' to get the host part
    let parts: Vec<&str> = after_protocol.split('/').collect();
    if parts.is_empty() {
        return false;
    }

    let host_and_port = parts[0];

    // Split host and port if port exists
    let host = if host_and_port.contains(':') {
        let host_port: Vec<&str> = host_and_port.split(':').collect();
        if host_port.len() != 2 {
            return false;
        }

        // Validate port is numeric
        if let Err(_) = host_port[1].parse::<u16>() {
            return false;
        }

        host_port[0]
    } else {
        host_and_port
    };

    // Host must not be empty and must contain at least one dot or be localhost/127.0.0.1
    if host.is_empty() {
        return false;
    }

    // Allow localhost and IP addresses
    if host == "localhost" || is_ip_address(host) {
        return true;
    }

    // Must contain at least one dot for domain names
    host.contains('.')
}

fn is_ip_address(host: &str) -> bool {
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() != 4 {
        return false;
    }

    for part in parts {
        if let Ok(num) = part.parse::<u8>() {
            // Valid IP part (0-255)
            continue;
        } else {
            return false;
        }
    }

    true
}
```

## Key Concepts Explained

### 1. String Processing Without Regex

This implementation demonstrates how to parse URLs without external regex crates, using only Rust's standard library string methods:

- **`split_whitespace()`**: Efficiently splits text into words, handling multiple whitespace characters
- **`to_lowercase()`**: Provides case-insensitive protocol matching
- **`starts_with()`**: Fast prefix matching for protocol detection
- **`chars().last()`**: Safe character access for punctuation cleanup

### 2. Memory-Efficient Deduplication

```rust
let mut seen = std::collections::HashSet::new();
if seen.insert(normalized_url.clone()) {
    urls.push(normalized_url);
}
```

- Uses `HashSet` for O(1) duplicate detection
- `insert()` returns `false` if the item already exists
- Only clones strings when actually needed

### 3. URL Validation Strategy

The validation approach breaks down URL parsing into manageable steps:

1. **Protocol Validation**: Check for valid http/https prefixes
2. **Host Extraction**: Split on '/' to isolate the host portion
3. **Port Handling**: Parse and validate numeric ports when present
4. **Domain Validation**: Ensure proper domain structure or allow localhost/IPs

### 4. Edge Case Handling

- **Trailing Punctuation**: Removes common punctuation that appears after URLs in text
- **Case Sensitivity**: Normalizes protocols while preserving original URL case
- **Empty Inputs**: Safely handles empty strings and whitespace-only text
- **Malformed URLs**: Validates structure to avoid false positives

### 5. Iterator Patterns

The solution demonstrates idiomatic Rust iteration:

```rust
for word in text.split_whitespace() {
    // Process each word
}

while let Some(last_char) = result.chars().last() {
    // Safe character iteration with pattern matching
}
```

## Best Practices Demonstrated

### String vs &str Usage

- **Input Parameters**: Uses `&str` for flexibility (accepts both `String` and `&str`)
- **Return Values**: Uses `String` for owned data that outlives the function
- **Intermediate Processing**: Creates owned `String` only when modification is needed

### UTF-8 Safety

Rust's string handling is UTF-8 safe by default:

- `chars()` iterates over Unicode scalar values, not bytes
- `split_whitespace()` correctly handles Unicode whitespace
- No risk of splitting UTF-8 sequences incorrectly

### Error Prevention

- Uses pattern matching (`match`, `if let`) to handle optional values safely
- Validates input at each step to prevent panics
- Returns empty collections rather than panicking on invalid input

### Performance Considerations

- **Single Pass**: Processes text in one iteration
- **Lazy Evaluation**: Uses iterators that process items on demand
- **Minimal Cloning**: Only clones strings when adding to result collection
- **Efficient Collections**: Uses `HashSet` for deduplication and `Vec` for ordered results

This implementation showcases fundamental Rust string manipulation techniques while building a practical text processing tool. The approach emphasizes safety, efficiency, and maintainability over raw performance.

