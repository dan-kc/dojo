# Character Frequency Counter Solution

## Implementation

```rust
pub fn count_char_frequencies(text: &str) -> std::collections::HashMap<char, usize> {
    let mut frequencies = std::collections::HashMap::new();
    
    for ch in text.chars() {
        frequencies.entry(ch).and_modify(|count| *count += 1).or_insert(1);
    }
    
    frequencies
}
```

## Alternative Implementation (Using or_insert)

```rust
pub fn count_char_frequencies(text: &str) -> std::collections::HashMap<char, usize> {
    let mut frequencies = std::collections::HashMap::new();
    
    for ch in text.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }
    
    frequencies
}
```

## Explanation

This solution demonstrates efficient character frequency counting using the HashMap Entry API:

1. **Entry API efficiency**: `entry()` provides a single hash lookup for both checking existence and modification
2. **and_modify() + or_insert()**: Cleanly handles both increment and initialization cases
3. **Unicode support**: `chars()` iterator properly handles Unicode characters including multi-byte sequences
4. **Single pass**: O(n) time complexity with one iteration through the text

## Key Learning Points

- **Entry API performance**: Avoids double hash lookups that would occur with `contains_key()` + `insert()`
- **Method chaining**: `and_modify().or_insert()` provides elegant conditional logic
- **Unicode handling**: Rust's `char` type correctly handles Unicode code points
- **Mutability patterns**: Dereferencing `count` in the closure to modify the stored value

## Rust Concepts Demonstrated

- HashMap Entry API (`entry()`, `and_modify()`, `or_insert()`)
- Iterator processing with `chars()`
- Closure-based value modification
- Unicode text processing
- Efficient collection operations and performance optimization