# Iterator Basics - Solution

## Solution

```rust
pub fn sum_evens(numbers: Vec<i32>) -> i32 {
    numbers
        .into_iter()
        .filter(|&n| n % 2 == 0)
        .sum()
}

pub fn filter_long_strings(strings: &[&str]) -> Vec<String> {
    strings
        .iter()
        .filter(|&s| s.len() > 3)
        .map(|&s| s.to_string())
        .collect()
}

pub fn first_five_divisible_by_three(start: i32, end: i32) -> Vec<i32> {
    (start..=end)
        .filter(|&n| n % 3 == 0)
        .take(5)
        .collect()
}

pub fn positive_numbers_to_strings(numbers: Vec<i32>) -> Vec<String> {
    numbers
        .into_iter()
        .filter(|&n| n > 0)
        .map(|n| n.to_string())
        .collect()
}
```

## Explanation

### Key Concepts

**Iterator Creation:**
- `into_iter()` consumes the collection and yields owned values
- `iter()` borrows the collection and yields references
- Ranges like `(start..=end)` are iterators by default

**Method Chaining:**
Iterators use method chaining for transformation pipelines. Each method returns a new iterator, enabling fluent composition.

**Lazy Evaluation:**
Iterator adaptors like `filter()` and `map()` are lazy - they do nothing until consumed by methods like `collect()`, `sum()`, or `take()`.

**Pattern Matching in Closures:**
- `|&n|` destructures references to get owned values
- `|n|` works with owned values directly
- `|&s|` in filter_long_strings gets the string slice from `&&str`

**Performance Notes:**
- `sum()` is a consuming iterator method that's highly optimized
- `collect()` allocates a new collection based on the iterator's size hint
- Method chaining avoids intermediate collections, improving performance

**Common Patterns:**
1. Filter then map: `filter().map().collect()`
2. Take limited results: `filter().take(n).collect()`
3. Transform and aggregate: `map().sum()` or `filter().sum()`