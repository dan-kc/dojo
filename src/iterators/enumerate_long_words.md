# iterators/enumerate_long_words

Reference implementation. Read after attempting the exercise.

```rust
pub fn enumerate_long_words(
    words: Vec<String>,
    skip: usize,
    min_chars: usize,
) -> Vec<(usize, String, usize)> {
    words
        .into_iter()
        .enumerate()
        .skip(skip)
        .filter_map(|(i, word)| {
            let count = word.chars().count();
            (count > min_chars).then_some((i, word, count))
        })
        .collect()
}
```
