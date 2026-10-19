# string_manipulation/word_counter

Reference implementation. Read after attempting the exercise.

```rust
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
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut word_count = 0;
    let mut letters = 0;
    for word in text
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
    {
        word_count += 1;
        letters += word.chars().count();
        *counts.entry(word.to_lowercase()).or_default() += 1;
    }
    let most_common_word = counts
        .iter()
        .min_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)))
        .map(|(w, _)| w.clone());
    WordStats {
        word_count,
        unique_words: counts.len(),
        character_count: text.chars().count(),
        character_count_no_spaces: text.chars().filter(|c| !c.is_whitespace()).count(),
        line_count: text.lines().count(),
        average_word_length: if word_count == 0 {
            0.0
        } else {
            letters as f64 / word_count as f64
        },
        most_common_word,
    }
}
```
