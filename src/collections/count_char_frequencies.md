# Character Frequency Counting

## Solution

```rust
use std::collections::HashMap;

fn count_char_frequencies(text: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    
    for ch in text.chars() {
        frequencies.entry(ch)
                   .and_modify(|count| *count += 1)
                   .or_insert(1);
    }
    
    frequencies
}
```

## Alternative with or_insert

```rust
use std::collections::HashMap;

fn count_char_frequencies(text: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    
    for ch in text.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }
    
    frequencies
}
```

## Functional Style Implementation

```rust
use std::collections::HashMap;

fn count_char_frequencies(text: &str) -> HashMap<char, usize> {
    text.chars()
        .fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
}
```

## With Capacity Optimization

```rust
use std::collections::HashMap;

fn count_char_frequencies(text: &str) -> HashMap<char, usize> {
    // Estimate capacity based on text length (assuming some duplicate characters)
    let estimated_unique_chars = (text.len() / 2).max(1).min(256);
    let mut frequencies = HashMap::with_capacity(estimated_unique_chars);
    
    for ch in text.chars() {
        match frequencies.entry(ch) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(1);
            }
        }
    }
    
    frequencies
}
```

## Case-Insensitive Version

```rust
use std::collections::HashMap;

fn count_char_frequencies_case_insensitive(text: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    
    for ch in text.chars() {
        let lowercase_ch = ch.to_lowercase().next().unwrap_or(ch);
        *frequencies.entry(lowercase_ch).or_insert(0) += 1;
    }
    
    frequencies
}
```

## With Filtering

```rust
use std::collections::HashMap;

fn count_alphabetic_frequencies(text: &str) -> HashMap<char, usize> {
    let mut frequencies = HashMap::new();
    
    for ch in text.chars().filter(|c| c.is_alphabetic()) {
        frequencies.entry(ch.to_lowercase().next().unwrap_or(ch))
                   .and_modify(|count| *count += 1)
                   .or_insert(1);
    }
    
    frequencies
}
```

## Explanation

The character frequency counting function demonstrates efficient HashMap usage patterns:

1. **Entry API**: The `entry()` method provides the most efficient way to insert or update values, avoiding double lookups.

2. **and_modify() + or_insert()**: This pattern handles both cases:
   - If key exists: increment the existing count
   - If key doesn't exist: insert with initial value of 1

3. **or_insert() Pattern**: Simpler alternative that inserts 0 for new keys, then increments.

4. **Memory Efficiency**: Using `HashMap::with_capacity()` can reduce allocations if you can estimate the number of unique characters.

5. **Pattern Matching**: Using explicit `Entry::Occupied` and `Entry::Vacant` provides maximum control over the insertion logic.

6. **Functional Approach**: Using `fold()` creates a more functional programming style, building the HashMap through iteration.

Key advantages of the Entry API:
- **Single Hash Lookup**: Only hashes the key once per operation
- **Atomic Updates**: Avoids race conditions in concurrent scenarios
- **Ergonomic**: Clean, readable code that expresses intent clearly
- **Efficient**: Optimal performance for insert-or-update patterns

This pattern is fundamental to Rust HashMap usage and appears frequently in real-world code for counting, caching, and memoization scenarios.