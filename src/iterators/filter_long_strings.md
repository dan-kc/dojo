# Filter Long Strings Using Iterators

## Solution

```rust
pub fn filter_long_strings(strings: &[&str]) -> Vec<String> {
    strings
        .iter()
        .filter(|&s| s.len() > 3)
        .map(|&s| s.to_string())
        .collect()
}
```

## Explanation

This solution demonstrates **iterator chaining with filtering and transformation**:

### Key Concepts Demonstrated:

1. **Slice Iteration**:
   - `strings.iter()` creates an iterator over `&&str` (references to string slices)
   - Works with borrowed data, doesn't take ownership of the slice
   - Efficient - no data copying during iteration

2. **Predicate Filtering**:
   - `filter(|&s| s.len() > 3)` uses pattern matching to destructure `&&str` to `&str`
   - Filters based on string length using `len()` method
   - Only strings longer than 3 characters pass through

3. **Type Transformation**:
   - `map(|&s| s.to_string())` converts `&str` to owned `String`
   - Pattern matching destructures `&&str` to `&str`, then calls `to_string()`
   - Creates owned data suitable for returning from the function

4. **Collection Building**:
   - `collect()` materializes the iterator into a `Vec<String>`
   - Type inference determines the target collection type
   - Allocates and builds the final result

### Iterator Type Flow:

```rust
&[&str]                    // Input slice
  .iter()               → Iterator<Item = &&str>
  .filter(|&s| s.len() > 3) → Filter<Iterator, Closure>
  .map(|&s| s.to_string())  → Map<Filter<Iterator, Closure>, Closure>
  .collect()            → Vec<String>
```

### Pattern Matching in Closures:

```rust
.filter(|&s| s.len() > 3)  // Pattern match &&str → &str
.map(|&s| s.to_string())   // Pattern match &&str → &str

// Alternative syntax without pattern matching:
.filter(|s| s.len() > 3)   // Work with &&str directly  
.map(|s| s.to_string())    // Call to_string() on &&str (Deref coercion)
```

### Why This Chain Is Efficient:

1. **Lazy Evaluation**: No work is done until `collect()` is called
2. **Single Pass**: Data flows through all transformations in one iteration
3. **No Intermediate Allocations**: No temporary vectors created between operations
4. **Memory Efficient**: Only the final result is allocated

### String Conversion Details:

```rust
// &str → String conversion options:
s.to_string()    // Most common, clear intent
s.to_owned()     // Explicit about creating owned data
String::from(s)  // Alternative constructor syntax
format!("{}", s) // Overkill but works
```

### Handling Empty Input:

```rust
let empty: &[&str] = &[];
let result = filter_long_strings(empty);
assert_eq!(result, Vec::<String>::new()); // Returns empty vector
```

### Unicode and Length Considerations:

```rust
// len() returns byte count, not character count:
assert_eq!("café".len(), 5);  // 4 chars + 1 for é in UTF-8
assert_eq!("café".chars().count(), 4);  // Actual character count

// For character-based filtering:
pub fn filter_long_strings_chars(strings: &[&str]) -> Vec<String> {
    strings
        .iter()
        .filter(|&s| s.chars().count() > 3)  // Character count
        .map(|&s| s.to_string())
        .collect()
}
```

### Alternative Implementations:

```rust
// Using filter_map for combined filter + map:
pub fn filter_long_strings_v2(strings: &[&str]) -> Vec<String> {
    strings
        .iter()
        .filter_map(|&s| {
            if s.len() > 3 {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect()
}

// Using retain pattern (if input was Vec<String>):
pub fn filter_long_strings_retain(mut strings: Vec<String>) -> Vec<String> {
    strings.retain(|s| s.len() > 3);
    strings
}

// Manual loop (for comparison):
pub fn filter_long_strings_manual(strings: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for &s in strings {
        if s.len() > 3 {
            result.push(s.to_string());
        }
    }
    result
}
```

### Performance Analysis:

- **Time Complexity**: O(n) where n is the number of input strings
- **Space Complexity**: O(k) where k is the number of strings that pass the filter
- **Allocation**: Single allocation for the result vector (with potential resizing)

### Memory Layout:

```
Input:  &[&str]     [ptr, ptr, ptr, ptr] → ["hi", "hello", "rust", "a"]
                           ↓ (filter & map)
Output: Vec<String> [String, String, String] = ["hello", "rust"] (owned data)
```

### Real-World Applications:

- **Input Validation**: Filtering user input by length requirements
- **Data Processing**: Cleaning datasets based on content length
- **Text Analysis**: Preprocessing text by removing short words
- **Configuration**: Filtering valid configuration values

This example showcases how iterator chaining enables concise, readable code while maintaining excellent performance through lazy evaluation and minimal allocations.