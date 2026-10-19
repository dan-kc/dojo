# iterators/top_k_frequent_words

Reference implementation. Read after attempting the exercise.

```rust
pub fn top_k_frequent_words(text: &str, k: usize) -> Vec<String> {
    use std::collections::HashMap;

    if text.is_empty() {
        return vec![];
    }

    let mut word_counts = HashMap::new();

    text.split_whitespace().for_each(|word| {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    });

    let mut sorted: Vec<_> = word_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    sorted.into_iter().take(k).map(|(word, _)| word).collect()
}
```
