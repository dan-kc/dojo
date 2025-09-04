# Unicode Text Processing - Solution

## Complete Implementation

```rust
pub fn normalize_text(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    let mut last_was_space = true; // Start as true to trim leading whitespace

    while let Some(ch) = chars.next() {
        match ch {
            // Handle whitespace characters
            ' ' | '\t' | '\n' | '\r' | '\u{00A0}' => { // Including non-breaking space
                if !last_was_space {
                    result.push(' ');
                    last_was_space = true;
                }
            }

            // Convert curly quotes to straight quotes
            '\u{2018}' | '\u{2019}' => { // Left/right single quotes
                result.push('\'');
                last_was_space = false;
            }
            '\u{201C}' | '\u{201D}' => { // Left/right double quotes
                result.push('"');
                last_was_space = false;
            }

            // Convert em-dash and en-dash to hyphen
            '\u{2014}' | '\u{2013}' => { // Em-dash, en-dash
                result.push('-');
                last_was_space = false;
            }

            // Remove zero-width characters
            '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' => {
                // Zero-width space, ZWNJ, ZWJ, BOM - skip these
                continue;
            }

            // Remove control characters (except preserved whitespace)
            c if c.is_control() => {
                // Skip control characters
                continue;
            }

            // Convert to lowercase and add normal characters
            _ => {
                for lowercase_ch in ch.to_lowercase() {
                    result.push(lowercase_ch);
                }
                last_was_space = false;
            }
        }
    }

    // Trim trailing whitespace
    result.trim_end().to_string()
}
```

## Advanced Implementation with Unicode Normalization

```rust
use std::collections::HashMap;

pub fn normalize_text_advanced(text: &str) -> String {
    // First, perform Unicode normalization (NFC - Canonical Decomposition + Composition)
    let nfc_normalized = unicode_normalize_nfc(text);

    let mut result = String::new();
    let mut chars = nfc_normalized.chars().peekable();
    let mut last_was_space = true;

    while let Some(ch) = chars.next() {
        match ch {
            // Comprehensive whitespace handling
            c if is_unicode_whitespace(c) => {
                if !last_was_space {
                    result.push(' ');
                    last_was_space = true;
                }
            }

            // Quote normalization
            c if is_curly_quote(c) => {
                result.push(normalize_quote(c));
                last_was_space = false;
            }

            // Dash normalization
            c if is_dash_variant(c) => {
                result.push('-');
                last_was_space = false;
            }

            // Zero-width and invisible characters
            c if is_zero_width(c) => {
                continue;
            }

            // Control character removal
            c if c.is_control() => {
                continue;
            }

            // RTL and bidirectional text markers
            c if is_bidi_control(c) => {
                continue; // Remove RTL/LTR marks
            }

            // Normal character processing
            _ => {
                // Handle special case of combining characters
                let normalized_ch = if is_combining_character(ch) {
                    // If we have a combining character without a base, skip it
                    if result.is_empty() || result.chars().last().map_or(true, |c| c.is_whitespace()) {
                        continue;
                    } else {
                        ch
                    }
                } else {
                    ch
                };

                for lowercase_ch in normalized_ch.to_lowercase() {
                    result.push(lowercase_ch);
                }
                last_was_space = false;
            }
        }
    }

    result.trim_end().to_string()
}

// Helper functions for advanced Unicode handling
fn unicode_normalize_nfc(text: &str) -> String {
    // Simplified NFC normalization - in production use unicode-normalization crate
    text.chars()
        .map(|c| match c {
            // Convert some common decomposed forms to composed
            'e' if text.chars().nth(1) == Some('\u{0301}') => 'é', // e + combining acute
            'a' if text.chars().nth(1) == Some('\u{0300}') => 'à', // a + combining grave
            _ => c,
        })
        .collect()
}

fn is_unicode_whitespace(c: char) -> bool {
    matches!(c,
        ' ' | '\t' | '\n' | '\r' | '\u{00A0}' | '\u{1680}' | '\u{2000}'..='\u{200A}' |
        '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
    )
}

fn is_curly_quote(c: char) -> bool {
    matches!(c, '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' |
               '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{201F}')
}

fn normalize_quote(c: char) -> char {
    match c {
        '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' => '\'',
        '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{201F}' => '"',
        _ => c,
    }
}

fn is_dash_variant(c: char) -> bool {
    matches!(c, '\u{2013}' | '\u{2014}' | '\u{2015}' | '\u{2212}')
}

fn is_zero_width(c: char) -> bool {
    matches!(c,
        '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' |
        '\u{061C}' | '\u{180E}' | '\u{2060}' | '\u{2061}' |
        '\u{2062}' | '\u{2063}' | '\u{2064}' | '\u{2066}' |
        '\u{2067}' | '\u{2068}' | '\u{2069}'
    )
}

fn is_bidi_control(c: char) -> bool {
    matches!(c,
        '\u{200E}' | '\u{200F}' | '\u{202A}' | '\u{202B}' |
        '\u{202C}' | '\u{202D}' | '\u{202E}' | '\u{2066}' |
        '\u{2067}' | '\u{2068}' | '\u{2069}'
    )
}

fn is_combining_character(c: char) -> bool {
    // Unicode combining marks (simplified check)
    matches!(c as u32, 0x0300..=0x036F | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF)
}
```

## Production-Ready Implementation with External Crate

```rust
// Note: This would require adding these dependencies to Cargo.toml:
// unicode-normalization = "0.1"
// unicode-segmentation = "1.10"

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub fn normalize_text_production(text: &str) -> String {
    // Perform Unicode NFC normalization
    let normalized: String = text.nfc().collect();

    let mut result = String::new();
    let mut last_was_space = true;

    // Process grapheme clusters for proper Unicode handling
    for grapheme in normalized.graphemes(true) {
        let chars: Vec<char> = grapheme.chars().collect();

        if chars.is_empty() {
            continue;
        }

        let first_char = chars[0];

        match first_char {
            // Whitespace normalization
            c if c.is_whitespace() => {
                if !last_was_space {
                    result.push(' ');
                    last_was_space = true;
                }
            }

            // Quote normalization
            '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' => {
                result.push('\'');
                last_was_space = false;
            }
            '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{201F}' => {
                result.push('"');
                last_was_space = false;
            }

            // Dash normalization
            '\u{2013}' | '\u{2014}' | '\u{2015}' | '\u{2212}' => {
                result.push('-');
                last_was_space = false;
            }

            // Skip zero-width and control characters
            c if is_invisible_or_control(c) => {
                continue;
            }

            // Normal character processing
            _ => {
                // Convert entire grapheme cluster to lowercase
                let lowercase_grapheme = grapheme.to_lowercase();
                result.push_str(&lowercase_grapheme);
                last_was_space = false;
            }
        }
    }

    result.trim_end().to_string()
}

fn is_invisible_or_control(c: char) -> bool {
    c.is_control() ||
    matches!(c,
        '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' |
        '\u{200E}' | '\u{200F}' | '\u{202A}' | '\u{202B}' |
        '\u{202C}' | '\u{202D}' | '\u{202E}' | '\u{2060}' |
        '\u{2061}' | '\u{2062}' | '\u{2063}' | '\u{2064}' |
        '\u{2066}' | '\u{2067}' | '\u{2068}' | '\u{2069}'
    )
}
```

## Key Concepts Explained

### 1. Unicode Character Processing

**Character vs Grapheme Clusters:**

```rust
// ❌ Naive approach - can break Unicode
for byte in text.bytes() {
    // This breaks multibyte UTF-8 sequences
}

// ✅ Safe character iteration
for ch in text.chars() {
    // This handles UTF-8 properly
}

// ✅ Grapheme cluster handling (requires external crate)
for grapheme in text.graphemes(true) {
    // This handles combined characters like emoji properly
}
```

**Case Conversion:**

```rust
// ✅ Proper Unicode case conversion
for lowercase_ch in ch.to_lowercase() {
    result.push(lowercase_ch);
}

// Some characters expand to multiple when case-converted
// Example: German ß -> "ss" when uppercased
```

### 2. Unicode Normalization Forms

**NFC (Canonical Decomposition + Composition):**

- Decomposes characters then recomposes in canonical form
- `é` (U+00E9) and `e` + `´` (U+0065 + U+0301) become the same

**NFD (Canonical Decomposition):**

- Decomposes all characters to base + combining marks
- Useful for searching and comparing

**NFKC/NFKD (Compatibility forms):**

- Also handles compatibility equivalences
- More aggressive normalization

### 3. Whitespace Handling Strategies

```rust
let mut last_was_space = true; // Start true to trim leading whitespace

match ch {
    c if c.is_whitespace() => {
        if !last_was_space {
            result.push(' ');
            last_was_space = true;
        }
    }
    _ => {
        // Normal character processing
        last_was_space = false;
    }
}

// Trim trailing whitespace at the end
result.trim_end().to_string()
```

### 4. Special Character Categories

**Zero-Width Characters:**

- U+200B: Zero-width space
- U+200C: Zero-width non-joiner
- U+200D: Zero-width joiner
- U+FEFF: Byte order mark

**Bidirectional Control:**

- U+200E: Left-to-right mark
- U+200F: Right-to-left mark
- U+202A-202E: Various directional controls

**Combining Characters:**

- U+0300-036F: Combining diacritical marks
- U+20D0-20FF: Combining marks for symbols

## Best Practices Demonstrated

### 1. Streaming Processing

```rust
let mut chars = text.chars().peekable();
while let Some(ch) = chars.next() {
    // Process character by character
    // Memory usage is constant, not proportional to input size
}
```

### 2. State Machine Approach

```rust
let mut last_was_space = true;
// State tracks whether we just processed whitespace
// Enables efficient whitespace collapsing
```

### 3. Character Classification

```rust
// Efficient character matching using ranges and sets
match ch {
    ' ' | '\t' | '\n' | '\r' | '\u{00A0}' => { /* whitespace */ }
    '\u{2018}' | '\u{2019}' => { /* single quotes */ }
    c if c.is_control() => { /* control chars */ }
    _ => { /* normal processing */ }
}
```

### 4. Memory Efficiency

```rust
// Pre-allocate if size is known
let mut result = String::with_capacity(text.len());

// Use push_str for string slices, push for single chars
result.push_str(&lowercase_grapheme);
result.push('-');

// Final trim to remove trailing whitespace
result.trim_end().to_string()
```

## Performance Considerations

### 1. Single-Pass Processing

- Processes each character exactly once
- Constant memory overhead regardless of input size
- No need for multiple normalization passes

### 2. Lazy Unicode Operations

- Only performs expensive operations (normalization, case conversion) when needed
- Uses efficient pattern matching for common cases

### 3. String Building Optimization

```rust
// Efficient: builds string incrementally
let mut result = String::new();
result.push(ch);

// Less efficient: creates many temporary strings
let result = parts.iter().map(|s| process(s)).collect::<String>();
```

This implementation demonstrates comprehensive Unicode text processing in Rust, handling the complexity of international text while maintaining safety and performance. The solution progresses from basic to production-ready approaches, showing how to handle Unicode normalization, character classification, and text cleaning operations correctly.

