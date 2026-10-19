# collections/count_char_frequencies

Reference implementation. Read after attempting the exercise.

```rust
use std::collections::HashMap;

fn count_char_frequencies(text: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();

    for ch in text.chars() {
        frequencies
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    frequencies
}
```
