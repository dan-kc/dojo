# collections/spell_check

Reference implementation. Read after attempting the exercise.

```rust
pub fn spell_check(
    text: &str,
    dictionary: &std::collections::HashSet<String>,
) -> std::collections::HashSet<String> {
    text.split(|c: char| !c.is_alphabetic())
        .filter(|word| !word.is_empty() && !dictionary.contains(*word))
        .map(str::to_owned)
        .collect()
}
```
