# String Parsing and Validation - Solution

## Complete Implementation

```rust
use std::collections::HashMap;

pub fn parse_key_value_pairs(input: &str) -> Result<HashMap<String, String>, String> {
    let mut result = HashMap::new();

    for (line_num, line) in input.lines().enumerate() {
        let trimmed_line = line.trim();

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Find the equals sign
        let equals_pos = trimmed_line.find('=')
            .ok_or_else(|| {
                format!("Line {}: missing '=' separator in '{}'", line_num + 1, trimmed_line)
            })?;

        // Split into key and value parts
        let key_part = trimmed_line[..equals_pos].trim();
        let value_part = trimmed_line[equals_pos + 1..].trim();

        // Validate key is not empty
        if key_part.is_empty() {
            return Err(format!("Line {}: empty key not allowed", line_num + 1));
        }

        // Insert the key-value pair
        result.insert(key_part.to_string(), value_part.to_string());
    }

    Ok(result)
}
```

## Alternative Implementation with Custom Error Type

```rust
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    MissingSeparator { line: usize, content: String },
    EmptyKey { line: usize },
    InvalidFormat { line: usize, reason: String },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingSeparator { line, content } => {
                write!(f, "Line {}: missing '=' separator in '{}'", line, content)
            }
            ParseError::EmptyKey { line } => {
                write!(f, "Line {}: empty key not allowed", line)
            }
            ParseError::InvalidFormat { line, reason } => {
                write!(f, "Line {}: {}", line, reason)
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_key_value_pairs_typed(input: &str) -> Result<HashMap<String, String>, ParseError> {
    let mut result = HashMap::new();

    for (line_num, line) in input.lines().enumerate() {
        let line_number = line_num + 1; // Human-readable line numbering
        let trimmed_line = line.trim();

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Find the equals sign
        let equals_pos = trimmed_line.find('=')
            .ok_or(ParseError::MissingSeparator {
                line: line_number,
                content: trimmed_line.to_string(),
            })?;

        // Extract key and value
        let key_part = trimmed_line[..equals_pos].trim();
        let value_part = trimmed_line[equals_pos + 1..].trim();

        // Validate key
        if key_part.is_empty() {
            return Err(ParseError::EmptyKey { line: line_number });
        }

        // Insert the pair
        result.insert(key_part.to_string(), value_part.to_string());
    }

    Ok(result)
}
```

## Advanced Parser with Additional Features

```rust
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseOptions {
    pub allow_duplicate_keys: bool,
    pub case_sensitive_keys: bool,
    pub allow_empty_values: bool,
    pub comment_chars: Vec<char>,
    pub separator_chars: Vec<char>,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            allow_duplicate_keys: false,
            case_sensitive_keys: true,
            allow_empty_values: true,
            comment_chars: vec!['#'],
            separator_chars: vec!['='],
        }
    }
}

pub fn parse_key_value_advanced(
    input: &str,
    options: &ParseOptions
) -> Result<HashMap<String, String>, String> {
    let mut result = HashMap::new();

    for (line_num, line) in input.lines().enumerate() {
        let line_number = line_num + 1;
        let trimmed_line = line.trim();

        // Skip empty lines
        if trimmed_line.is_empty() {
            continue;
        }

        // Skip comment lines
        if options.comment_chars.iter().any(|&c| trimmed_line.starts_with(c)) {
            continue;
        }

        // Find separator
        let separator_pos = options.separator_chars.iter()
            .find_map(|&sep| trimmed_line.find(sep))
            .ok_or_else(|| {
                format!("Line {}: missing separator in '{}'", line_number, trimmed_line)
            })?;

        // Extract key and value
        let key_part = trimmed_line[..separator_pos].trim();
        let value_part = trimmed_line[separator_pos + 1..].trim();

        // Validate key
        if key_part.is_empty() {
            return Err(format!("Line {}: empty key not allowed", line_number));
        }

        // Handle case sensitivity
        let final_key = if options.case_sensitive_keys {
            key_part.to_string()
        } else {
            key_part.to_lowercase()
        };

        // Check for duplicate keys
        if !options.allow_duplicate_keys && result.contains_key(&final_key) {
            return Err(format!("Line {}: duplicate key '{}'", line_number, final_key));
        }

        // Handle empty values
        if !options.allow_empty_values && value_part.is_empty() {
            return Err(format!("Line {}: empty value for key '{}'", line_number, final_key));
        }

        result.insert(final_key, value_part.to_string());
    }

    Ok(result)
}
```

## Key Concepts Explained

### 1. String Slicing and Parsing

```rust
let equals_pos = trimmed_line.find('=')
    .ok_or_else(|| format!("missing '=' separator"))?;

let key_part = trimmed_line[..equals_pos].trim();
let value_part = trimmed_line[equals_pos + 1..].trim();
```

**Critical Concepts:**

- **UTF-8 Safe Slicing**: `find()` returns byte positions, but Rust ensures slice boundaries are character boundaries
- **Range Syntax**: `[..pos]` and `[pos+1..]` for prefix and suffix extraction
- **Chained Operations**: `find().ok_or_else()` for converting `Option` to `Result`

### 2. Error Handling Patterns

**Simple String Errors:**

```rust
.ok_or_else(|| format!("Line {}: error description", line_num + 1))?;
```

**Custom Error Types:**

```rust
#[derive(Debug)]
pub enum ParseError {
    MissingSeparator { line: usize, content: String },
    EmptyKey { line: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingSeparator { line, content } => {
                write!(f, "Line {}: missing '=' in '{}'", line, content)
            }
            ParseError::EmptyKey { line } => {
                write!(f, "Line {}: empty key", line)
            }
        }
    }
}
```

### 3. Iterator Patterns for Text Processing

```rust
for (line_num, line) in input.lines().enumerate() {
    // Process each line with access to line number
}

// Alternative: functional approach
let result: Result<HashMap<String, String>, String> = input
    .lines()
    .enumerate()
    .filter(|(_, line)| !line.trim().is_empty() && !line.trim().starts_with('#'))
    .map(|(line_num, line)| parse_single_line(line, line_num + 1))
    .collect();
```

### 4. String vs &str Considerations

```rust
// Input parameter: &str for flexibility
pub fn parse_key_value_pairs(input: &str) -> Result<HashMap<String, String>, String>

// Key storage: String for owned data
result.insert(key_part.to_string(), value_part.to_string());

// Intermediate processing: &str for efficiency
let trimmed_line = line.trim(); // Returns &str
```

## Best Practices Demonstrated

### 1. Robust Input Validation

```rust
// Multiple validation layers
if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
    continue; // Skip gracefully
}

// Validate structure before processing
let equals_pos = trimmed_line.find('=')
    .ok_or_else(|| error_message)?;

// Validate semantics after parsing
if key_part.is_empty() {
    return Err(format!("empty key not allowed"));
}
```

### 2. Memory Efficient Processing

```rust
// Process line by line (streaming)
for (line_num, line) in input.lines().enumerate() {
    // Only allocate when needed
    let key = key_part.to_string(); // Allocate owned string
    let value = value_part.to_string();
    result.insert(key, value);
}
```

### 3. Error Context and Debugging

```rust
// Include line numbers in errors
format!("Line {}: missing '=' separator in '{}'", line_num + 1, trimmed_line)

// Provide context about what failed
return Err(format!("Line {}: empty key not allowed", line_number));
```

### 4. Flexible API Design

```rust
// Basic version for simple use cases
pub fn parse_key_value_pairs(input: &str) -> Result<HashMap<String, String>, String>

// Advanced version with options
pub fn parse_key_value_advanced(
    input: &str,
    options: &ParseOptions
) -> Result<HashMap<String, String>, String>

// Builder pattern for configuration
let options = ParseOptions::default()
    .with_case_insensitive()
    .with_custom_separator(':')
    .build();
```

## Performance Considerations

### 1. Single-Pass Processing

The parser processes input in a single pass:

- No need to scan the entire input multiple times
- Memory usage is proportional to the number of key-value pairs, not input size
- Suitable for streaming large configuration files

### 2. Efficient String Operations

```rust
// Efficient: reuses string slices where possible
let key_part = trimmed_line[..equals_pos].trim();

// Less efficient: creates intermediate strings
let parts: Vec<&str> = trimmed_line.split('=').collect();
let key_part = parts[0].trim();
```

### 3. HashMap Performance

- Uses Rust's default hasher (SipHash) for security
- For performance-critical parsing, consider `HashMap::with_capacity()` if size is known
- Keys are owned strings - necessary for the HashMap to outlive input

This implementation demonstrates idiomatic Rust text parsing with comprehensive error handling, efficiency, and flexibility. The approach emphasizes safety while providing clear error messages for debugging malformed input.

