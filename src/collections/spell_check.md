# HashSet Spell Checker Solution

## Implementation

```rust
pub fn spell_check(
    text: &str,
    dictionary: &std::collections::HashSet<String>,
) -> std::collections::HashSet<String> {
    let mut errors = std::collections::HashSet::new();
    
    for word in text.split_whitespace() {
        // Clean the word by removing punctuation
        let clean_word: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        
        if !clean_word.is_empty() && !dictionary.contains(&clean_word) {
            errors.insert(clean_word);
        }
    }
    
    errors
}
```

## Case-Insensitive Implementation

```rust
pub fn spell_check(
    text: &str,
    dictionary: &std::collections::HashSet<String>,
) -> std::collections::HashSet<String> {
    let mut errors = std::collections::HashSet::new();
    
    for word in text.split_whitespace() {
        let clean_word: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();
        
        if !clean_word.is_empty() {
            // Check if the original word (with original casing) is in dictionary
            if !dictionary.contains(word) && !dictionary.contains(&clean_word) {
                errors.insert(word.to_string());
            }
        }
    }
    
    errors
}
```

## Explanation

This solution implements a simple spell checker using HashSet:

1. **Text tokenization**: Splits input text into individual words
2. **Word cleaning**: Removes punctuation while preserving alphabetic characters
3. **Dictionary lookup**: O(1) average lookup time using HashSet
4. **Error collection**: Automatically deduplicates errors using HashSet

## Key Learning Points

- **Fast lookups**: HashSet provides O(1) average-case dictionary lookups
- **Text preprocessing**: Cleaning words by filtering characters
- **Automatic deduplication**: HashSet ensures each error appears only once
- **String processing**: Character filtering and case normalization

## Advanced Features

```rust
pub fn spell_check_advanced(
    text: &str,
    dictionary: &std::collections::HashSet<String>,
    case_sensitive: bool,
) -> std::collections::HashSet<String> {
    let mut errors = std::collections::HashSet::new();
    
    for word in text.split_whitespace() {
        let clean_word: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();
        
        if clean_word.is_empty() {
            continue;
        }
        
        let lookup_word = if case_sensitive {
            clean_word.clone()
        } else {
            clean_word.to_lowercase()
        };
        
        if !dictionary.contains(&lookup_word) {
            errors.insert(clean_word);
        }
    }
    
    errors
}
```

## Use Cases

- **Document validation**: Checking spelling in text documents
- **User input validation**: Real-time spell checking in text fields
- **Content processing**: Identifying misspelled words in large text corpora
- **Educational tools**: Highlighting spelling errors for learning

## Rust Concepts Demonstrated

- HashSet for fast membership testing
- String processing and character filtering
- Iterator methods for text tokenization
- Collection-based error accumulation
- Text normalization techniques