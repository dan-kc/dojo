# collections/word_index

Reference implementation. Read after attempting the exercise.

```rust
pub fn build_word_index(
    text: &str,
) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
    let mut index = std::collections::HashMap::new();

    for (position, word) in text.split_whitespace().enumerate() {
        index
            .entry(word.to_string())
            .or_insert_with(std::collections::HashSet::new)
            .insert(position);
    }

    index
}
```
