# Word Index Solution

## Implementation

```rust
pub fn build_word_index(text: &str) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
    let mut index = std::collections::HashMap::new();
    
    for (position, word) in text.split_whitespace().enumerate() {
        index.entry(word.to_string())
             .or_insert_with(std::collections::HashSet::new)
             .insert(position);
    }
    
    index
}
```

## Case-Insensitive Implementation

```rust
pub fn build_word_index(text: &str) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
    let mut index = std::collections::HashMap::new();
    
    for (position, word) in text.split_whitespace().enumerate() {
        let normalized_word = word.to_lowercase();
        index.entry(normalized_word)
             .or_insert_with(std::collections::HashSet::new)
             .insert(position);
    }
    
    index
}
```

## Explanation

This solution builds an inverted index for text search:

1. **Word tokenization**: Uses `split_whitespace()` to extract words
2. **Position tracking**: `enumerate()` provides word positions in the text
3. **HashSet for positions**: Efficiently stores unique positions for each word
4. **Entry API efficiency**: `or_insert_with()` creates new HashSets only when needed

## Key Learning Points

- **Inverted indexing**: Mapping from terms to their locations in a document
- **Nested collections**: HashMap containing HashSets as values
- **Position-based search**: Enabling queries about where words appear
- **Memory efficiency**: HashSet automatically handles duplicate positions

## Advanced Implementation with Punctuation Handling

```rust
pub fn build_word_index(text: &str) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
    let mut index = std::collections::HashMap::new();
    
    for (position, word) in text
        .split_whitespace()
        .enumerate() 
    {
        // Clean the word by removing punctuation and converting to lowercase
        let clean_word: String = word
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>()
            .to_lowercase();
        
        if !clean_word.is_empty() {
            index.entry(clean_word)
                 .or_insert_with(std::collections::HashSet::new)
                 .insert(position);
        }
    }
    
    index
}
```

## Use Cases

- **Full-text search**: Building search indexes for documents
- **Query optimization**: Fast word lookup and phrase matching
- **Analytics**: Analyzing word frequency and distribution
- **Concordance**: Creating word occurrence references

## Rust Concepts Demonstrated

- HashMap with HashSet values (nested collections)
- Entry API with lazy initialization (`or_insert_with()`)
- Iterator processing (`enumerate()`, `split_whitespace()`)
- String manipulation and normalization
- Text processing patterns and tokenization
- Efficient data structure composition