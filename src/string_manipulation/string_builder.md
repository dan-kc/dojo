# String Building and Query Construction - Solution

## Complete Implementation

```rust
use std::collections::HashMap;

pub fn build_query_string(params: &HashMap<String, Option<String>>) -> String {
    let mut pairs = Vec::new();

    // Collect valid key-value pairs (skip None values)
    for (key, value) in params {
        if let Some(val) = value {
            pairs.push((key.clone(), val.clone()));
        }
    }

    // Sort by key for consistent output
    pairs.sort_by(|a, b| a.0.cmp(&b.0));

    // Build query string
    pairs
        .iter()
        .map(|(key, value)| {
            format!("{}={}", url_encode(key), url_encode(value))
        })
        .collect::<Vec<String>>()
        .join("&")
}

fn url_encode(input: &str) -> String {
    let mut result = String::new();

    for byte in input.bytes() {
        match byte {
            // Unreserved characters (RFC 3986)
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            // Everything else gets percent-encoded
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }

    result
}
```

## Key Concepts Explained

### 1. Efficient String Building

This solution demonstrates several string building techniques:

**Using Iterator Patterns:**

```rust
pairs
    .iter()
    .map(|(key, value)| format!("{}={}", url_encode(key), url_encode(value)))
    .collect::<Vec<String>>()
    .join("&")
```

This approach:

- Builds each parameter pair lazily using `map()`
- Collects results into a vector
- Uses `join()` for efficient concatenation with separators

**Alternative Approaches:**

```rust
// Less efficient - multiple allocations
let mut result = String::new();
for (key, value) in &pairs {
    if !result.is_empty() {
        result.push('&');
    }
    result.push_str(&format!("{}={}", url_encode(key), url_encode(value)));
}

// More efficient for large datasets - pre-allocated capacity
let mut result = String::with_capacity(estimate_capacity(&pairs));
```

### 2. URL Encoding Implementation

The `url_encode` function demonstrates proper percent-encoding:

```rust
for byte in input.bytes() {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
            result.push(byte as char);
        }
        _ => {
            result.push_str(&format!("%{:02X}", byte));
        }
    }
}
```

**Key Points:**

- **Byte-level Processing**: Works with bytes rather than chars for proper encoding
- **RFC 3986 Compliance**: Only unreserved characters pass through unencoded
- **Hexadecimal Encoding**: Uses uppercase hex as per standard
- **UTF-8 Safe**: Rust's `bytes()` method handles UTF-8 correctly

### 3. HashMap Processing and Sorting

```rust
// Collect valid key-value pairs (skip None values)
for (key, value) in params {
    if let Some(val) = value {
        pairs.push((key.clone(), val.clone()));
    }
}

// Sort by key for consistent output
pairs.sort_by(|a, b| a.0.cmp(&b.0));
```

**Design Decisions:**

- **Option Handling**: `None` values are excluded, empty strings are included
- **Deterministic Output**: Sorting ensures consistent query string generation
- **Memory Management**: Clones strings only for valid pairs

### 4. Memory Efficiency Considerations

**Current Implementation:**

- Creates intermediate `Vec` for sorting
- Clones strings for owned data
- Uses efficient `join()` for final concatenation

**Optimization Opportunities:**

```rust
// Pre-calculate capacity to reduce allocations
fn build_query_string_optimized(params: &HashMap<String, Option<String>>) -> String {
    // Estimate final string length
    let estimated_len = params
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|val| k.len() + val.len() + 10)) // +10 for encoding overhead
        .sum::<usize>();

    let mut result = String::with_capacity(estimated_len);
    // ... rest of implementation
}
```

## Advanced String Building Patterns

### 1. Builder Pattern Implementation

```rust
pub struct QueryBuilder {
    params: HashMap<String, Option<String>>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn add<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.params.insert(key.into(), Some(value.into()));
        self
    }

    pub fn build(self) -> String {
        build_query_string(&self.params)
    }
}

// Usage:
let query = QueryBuilder::new()
    .add("name", "John Doe")
    .add("age", "30")
    .build();
```

### 2. Format Macro Alternatives

```rust
// Using write! for more control
use std::fmt::Write;

fn format_parameter(key: &str, value: &str) -> String {
    let mut result = String::new();
    write!(result, "{}={}", url_encode(key), url_encode(value)).unwrap();
    result
}

// Using format! with better error handling
fn format_parameter_safe(key: &str, value: &str) -> Result<String, std::fmt::Error> {
    Ok(format!("{}={}", url_encode(key), url_encode(value)))
}
```

## Best Practices Demonstrated

### 1. String vs &str Usage

- **Parameters**: Accept `&HashMap` to avoid unnecessary ownership transfer
- **Return Values**: Return `String` for owned, modified data
- **Internal Processing**: Use `&str` when possible to avoid allocations

### 2. Error Handling Strategies

```rust
// Explicit Option handling
if let Some(val) = value {
    pairs.push((key.clone(), val.clone()));
}

// Alternative: using filter_map
let pairs: Vec<_> = params
    .iter()
    .filter_map(|(k, v)| v.as_ref().map(|val| (k.clone(), val.clone())))
    .collect();
```

### 3. Performance Considerations

- **Lazy Evaluation**: Use iterators when possible
- **Minimal Cloning**: Clone only when ownership is required
- **Efficient Joining**: Use `join()` instead of manual concatenation
- **Capacity Planning**: Consider pre-allocating string capacity for large datasets

### 4. Unicode and UTF-8 Handling

The implementation correctly handles Unicode by:

- Processing bytes for encoding (not chars)
- Preserving UTF-8 sequences during percent-encoding
- Using Rust's built-in UTF-8 safety guarantees

This solution showcases efficient string building techniques while maintaining correctness, performance, and Unicode safety. The approach emphasizes functional programming patterns and Rust's ownership system to create maintainable, safe code.

