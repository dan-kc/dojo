# Top K Frequent Words

## Solution

```rust
pub fn top_k_frequent_words(text: &str, k: usize) -> Vec<String> {
    use std::collections::HashMap;
    
    if text.is_empty() {
        return vec![];
    }
    
    let mut word_counts = HashMap::new();
    
    // Count word frequencies
    text.split_whitespace()
        .for_each(|word| {
            *word_counts.entry(word.to_string()).or_insert(0) += 1;
        });
    
    // Sort by frequency and take top k
    let mut sorted: Vec<_> = word_counts.into_iter().collect();
    sorted.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });
    
    sorted.into_iter()
        .take(k)
        .map(|(word, _)| word)
        .collect()
}
```

## Explanation

This solution efficiently processes text to find the most frequent words:

1. **Word Splitting**: `split_whitespace()` efficiently tokenizes the text into words
2. **Frequency Counting**: Uses a HashMap to count occurrences of each word
3. **Sorting Strategy**: 
   - Primary: Sort by frequency (descending)
   - Secondary: Sort alphabetically for consistent ordering when frequencies are equal
4. **Top K Selection**: Uses `take(k)` to limit results to the requested number

Key optimizations:
- **Early Return**: Handles empty text without unnecessary processing
- **In-place Counting**: Uses `entry().or_insert()` pattern for efficient HashMap updates
- **Iterator Chain**: Final transformation uses iterator methods to avoid intermediate collections
- **Stable Sorting**: The secondary alphabetical sort ensures deterministic results when words have equal frequency

This approach balances readability with performance, using Rust's iterator methods to process text efficiently.