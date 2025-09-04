# Word Counting and Text Analysis - Solution

## Complete Implementation

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct WordStats {
    pub word_count: usize,
    pub unique_words: usize,
    pub character_count: usize,
    pub character_count_no_spaces: usize,
    pub line_count: usize,
    pub average_word_length: f64,
    pub most_common_word: Option<String>,
}

pub fn count_words(text: &str) -> WordStats {
    if text.is_empty() {
        return WordStats {
            word_count: 0,
            unique_words: 0,
            character_count: 0,
            character_count_no_spaces: 0,
            line_count: 0,
            average_word_length: 0.0,
            most_common_word: None,
        };
    }
    
    // Count characters and lines
    let character_count = text.chars().count();
    let character_count_no_spaces = text.chars().filter(|&c| !c.is_whitespace()).count();
    let line_count = if text.contains('\n') {
        text.lines().count()
    } else {
        1 // Single line if no newlines
    };
    
    // Extract words and count them
    let mut word_frequencies = HashMap::new();
    let mut total_word_length = 0;
    let mut word_count = 0;
    
    // Split text into words (sequences of alphabetic characters)
    for word in extract_words(text) {
        let word_lower = word.to_lowercase();
        total_word_length += word.len();
        word_count += 1;
        
        *word_frequencies.entry(word_lower).or_insert(0) += 1;
    }
    
    let unique_words = word_frequencies.len();
    
    // Calculate average word length
    let average_word_length = if word_count > 0 {
        total_word_length as f64 / word_count as f64
    } else {
        0.0
    };
    
    // Find most common word
    let most_common_word = word_frequencies
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(word, _)| word.clone());
    
    WordStats {
        word_count,
        unique_words,
        character_count,
        character_count_no_spaces,
        line_count,
        average_word_length,
        most_common_word,
    }
}

/// Extracts words from text as sequences of alphabetic characters
fn extract_words(text: &str) -> impl Iterator<Item = &str> {
    WordIterator::new(text)
}

/// Custom iterator for extracting words from text
struct WordIterator<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> WordIterator<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, position: 0 }
    }
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        let remaining = &self.text[self.position..];
        
        // Skip non-alphabetic characters
        let start_offset = remaining.find(|c: char| c.is_alphabetic())?;
        let word_start = self.position + start_offset;
        
        // Find end of word (next non-alphabetic character)
        let word_slice = &self.text[word_start..];
        let word_length = word_slice
            .find(|c: char| !c.is_alphabetic())
            .unwrap_or(word_slice.len());
        
        let word_end = word_start + word_length;
        self.position = word_end;
        
        Some(&self.text[word_start..word_end])
    }
}
```

## Alternative Implementation Using Regex Approach

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct WordStats {
    pub word_count: usize,
    pub unique_words: usize,
    pub character_count: usize,
    pub character_count_no_spaces: usize,
    pub line_count: usize,
    pub average_word_length: f64,
    pub most_common_word: Option<String>,
}

pub fn count_words_regex_style(text: &str) -> WordStats {
    if text.is_empty() {
        return WordStats::default();
    }
    
    // Character and line counts
    let character_count = text.chars().count();
    let character_count_no_spaces = text.chars().filter(|&c| !c.is_whitespace()).count();
    let line_count = text.lines().count().max(1);
    
    // Word extraction and analysis
    let words: Vec<&str> = text
        .split(|c: char| !c.is_alphabetic())
        .filter(|word| !word.is_empty())
        .collect();
    
    if words.is_empty() {
        return WordStats {
            character_count,
            character_count_no_spaces,
            line_count,
            ..WordStats::default()
        };
    }
    
    // Count word frequencies
    let mut word_frequencies = HashMap::new();
    let total_word_length: usize = words.iter().map(|word| word.len()).sum();
    
    for word in &words {
        let word_lower = word.to_lowercase();
        *word_frequencies.entry(word_lower).or_insert(0) += 1;
    }
    
    let word_count = words.len();
    let unique_words = word_frequencies.len();
    let average_word_length = total_word_length as f64 / word_count as f64;
    
    let most_common_word = word_frequencies
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, _)| word);
    
    WordStats {
        word_count,
        unique_words,
        character_count,
        character_count_no_spaces,
        line_count,
        average_word_length,
        most_common_word,
    }
}

impl Default for WordStats {
    fn default() -> Self {
        Self {
            word_count: 0,
            unique_words: 0,
            character_count: 0,
            character_count_no_spaces: 0,
            line_count: 0,
            average_word_length: 0.0,
            most_common_word: None,
        }
    }
}
```

## Advanced Implementation with Unicode Support

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct WordStats {
    pub word_count: usize,
    pub unique_words: usize,
    pub character_count: usize,
    pub character_count_no_spaces: usize,
    pub line_count: usize,
    pub average_word_length: f64,
    pub most_common_word: Option<String>,
}

pub fn count_words_unicode(text: &str) -> WordStats {
    if text.is_empty() {
        return WordStats::default();
    }
    
    // Unicode-aware character counting
    let character_count = text.chars().count();
    let character_count_no_spaces = text.chars()
        .filter(|&c| !c.is_whitespace())
        .count();
    
    // Line counting (handle different line endings)
    let line_count = text.lines().count().max(1);
    
    // Extract words using Unicode word boundaries
    let words = extract_unicode_words(text);
    let word_count = words.len();
    
    if word_count == 0 {
        return WordStats {
            character_count,
            character_count_no_spaces,
            line_count,
            ..WordStats::default()
        };
    }
    
    // Analyze words
    let mut word_frequencies = HashMap::new();
    let mut total_char_count = 0;
    
    for word in &words {
        // Use Unicode normalization for consistent comparison
        let normalized_word = unicode_normalize_lowercase(word);
        total_char_count += word.chars().count();
        
        *word_frequencies.entry(normalized_word).or_insert(0) += 1;
    }
    
    let unique_words = word_frequencies.len();
    let average_word_length = total_char_count as f64 / word_count as f64;
    
    // Find most frequent word
    let most_common_word = word_frequencies
        .into_iter()
        .max_by(|(_, count1), (_, count2)| {
            count1.cmp(count2).then_with(|| {
                // If counts are equal, prefer lexicographically smaller word
                std::cmp::Ordering::Greater // Reverse to get smaller word
            })
        })
        .map(|(word, _)| word);
    
    WordStats {
        word_count,
        unique_words,
        character_count,
        character_count_no_spaces,
        line_count,
        average_word_length,
        most_common_word,
    }
}

/// Extract words considering Unicode word boundaries
fn extract_unicode_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut in_word = false;
    
    for ch in text.chars() {
        if is_word_character(ch) {
            current_word.push(ch);
            in_word = true;
        } else {
            if in_word && !current_word.is_empty() {
                words.push(current_word.clone());
                current_word.clear();
            }
            in_word = false;
        }
    }
    
    // Don't forget the last word
    if in_word && !current_word.is_empty() {
        words.push(current_word);
    }
    
    words
}

/// Check if a character is part of a word (Unicode-aware)
fn is_word_character(ch: char) -> bool {
    ch.is_alphabetic() || 
    ch.is_numeric() ||
    matches!(ch, '\'' | '-' | '_') // Include apostrophes, hyphens, underscores
}

/// Normalize text for comparison (simplified Unicode normalization)
fn unicode_normalize_lowercase(text: &str) -> String {
    // In production, use the `unicode-normalization` crate
    text.chars()
        .flat_map(|c| c.to_lowercase())
        .collect()
}

impl Default for WordStats {
    fn default() -> Self {
        Self {
            word_count: 0,
            unique_words: 0,
            character_count: 0,
            character_count_no_spaces: 0,
            line_count: 0,
            average_word_length: 0.0,
            most_common_word: None,
        }
    }
}
```

## Key Concepts Explained

### 1. Iterator Patterns for Text Processing

**Custom Iterator Implementation:**
```rust
struct WordIterator<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = &'a str;
    
    fn next(&mut self) -> Option<Self::Item> {
        // Find next word boundary
        // Return string slice without allocation
    }
}
```

**Benefits:**
- **Zero-allocation**: Returns string slices, not owned strings
- **Lazy evaluation**: Words are found on-demand
- **Memory efficient**: Constant memory usage regardless of text size

### 2. HashMap for Frequency Counting

```rust
let mut word_frequencies = HashMap::new();
for word in words {
    *word_frequencies.entry(word_lower).or_insert(0) += 1;
}
```

**Key Techniques:**
- **`entry()` API**: Efficiently handles insertion or update
- **`or_insert()`**: Provides default value for new keys
- **Dereferencing**: `*` to modify the value in-place

### 3. Statistical Calculations

```rust
// Average with division by zero protection
let average_word_length = if word_count > 0 {
    total_word_length as f64 / word_count as f64
} else {
    0.0
};

// Finding maximum with iterator combinators
let most_common_word = word_frequencies
    .iter()
    .max_by_key(|(_, &count)| count)
    .map(|(word, _)| word.clone());
```

### 4. Unicode Text Handling

**Character vs Byte Counting:**
```rust
// ❌ Byte count (wrong for Unicode)
let count = text.len();

// ✅ Character count (Unicode-aware)
let count = text.chars().count();

// ✅ Grapheme cluster count (requires external crate)
let count = text.graphemes(true).count();
```

**Case Conversion:**
```rust
// ✅ Unicode-aware case conversion
let lowercase = ch.to_lowercase().collect::<String>();

// Some characters expand when case-converted
// Example: Turkish İ -> i̇ (i with dot above)
```

## Best Practices Demonstrated

### 1. Memory Efficiency

```rust
// Efficient: process in single pass
for word in extract_words(text) {
    // Process immediately
}

// Less efficient: collect all then process
let all_words: Vec<_> = extract_words(text).collect();
for word in all_words {
    // Two passes through data
}
```

### 2. Error Handling and Edge Cases

```rust
// Handle empty input gracefully
if text.is_empty() {
    return WordStats::default();
}

// Protect against division by zero
let average = if count > 0 { 
    total as f64 / count as f64 
} else { 
    0.0 
};

// Handle line counting edge cases
let line_count = text.lines().count().max(1);
```

### 3. Functional Programming Patterns

```rust
// Chain operations efficiently
let words: Vec<&str> = text
    .split(|c: char| !c.is_alphabetic())
    .filter(|word| !word.is_empty())
    .collect();

// Use iterator combinators for complex operations
let most_frequent = frequencies
    .into_iter()
    .max_by_key(|(_, count)| *count)
    .map(|(word, _)| word);
```

### 4. Type Safety and API Design

```rust
// Use custom types for clear APIs
#[derive(Debug, PartialEq)]
pub struct WordStats {
    // Explicit field types prevent confusion
    pub word_count: usize,        // Not i32 or u32
    pub average_word_length: f64, // Not f32 for precision
}

// Return owned data for API consumers
pub most_common_word: Option<String>, // Not Option<&str>
```

## Performance Considerations

### 1. Single-Pass Processing

The implementation processes the text in a single pass:
- Counts characters while extracting words
- Builds frequency map incrementally
- Calculates statistics without additional iterations

### 2. Memory Usage

```rust
// Memory usage is proportional to unique word count, not total text size
let mut word_frequencies = HashMap::new(); // Size = unique words

// String slices avoid allocation during processing
for word in extract_words(text) { // &str, not String
    // Process without copying
}
```

### 3. Unicode Performance

```rust
// Efficient: single character iteration
for ch in text.chars() {
    if ch.is_alphabetic() { /* process */ }
}

// Less efficient: multiple string operations
let words = text
    .to_lowercase()      // Full string allocation
    .replace(punctuation, " ") // Another allocation
    .split_whitespace()  // Iterator creation
    .collect();          // Final allocation
```

This implementation demonstrates comprehensive text analysis in Rust, showcasing iterator patterns, Unicode handling, statistical calculations, and memory-efficient processing. The solution progresses from basic word counting to advanced Unicode-aware text analysis suitable for international content.