# Enumerate Long Words

## Solution

```rust
pub fn enumerate_long_words(words: Vec<String>) -> Vec<(usize, String, usize)> {
    words.into_iter()
        .skip(2)
        .enumerate()
        .filter(|(_, word)| word.len() > 4)
        .map(|(index, word)| {
            let len = word.len();
            (index, word, len)
        })
        .collect()
}
```

## Explanation

This solution demonstrates the importance of iterator combinator ordering:

1. **Skip**: Removes the first 2 elements from the iterator
2. **Enumerate**: Adds indices starting from 0 for the remaining elements
3. **Filter**: Keeps only words longer than 4 characters
4. **Map**: Transforms each item into a tuple with index, word, and character count

Key concepts:
- **Order Matters**: `skip()` before `enumerate()` means indices start at 0 after skipping
- **Tuple Destructuring**: Pattern matching in filter and map closures
- **Length Calculation**: Computed once and stored in the final tuple
- **Ownership Transfer**: `into_iter()` consumes the original vector

Important notes:
- The indices are relative to the position after skipping, not the original position
- Filter operates on the enumerated items, so we access the word via tuple destructuring
- This pattern is useful for processing subsets of data while maintaining position information

Common variations:
- Use `iter()` instead of `into_iter()` to keep the original vector
- Add the original index by calculating `index + 2` if needed
- Chain additional filters or transformations as needed