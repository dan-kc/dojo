# Process Owned String

## Solution

```rust
fn process_owned_string(s: String) -> usize {
    // Take ownership of the string and perform transformations
    let processed = s.trim() // Remove leading/trailing whitespace
                    .to_lowercase() // Convert to lowercase
                    .replace(" ", "_"); // Replace spaces with underscores
    
    // Return the length of the processed string
    processed.len()
}
```

## Alternative Implementation with More Processing

```rust
fn process_owned_string(mut s: String) -> usize {
    // Modify the string in place (we own it)
    s.push_str(" - processed");
    
    // Perform transformations
    let processed = s.trim()
                    .chars()
                    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                    .collect::<String>();
    
    processed.len()
}
```

## Version with Complex Processing

```rust
fn process_owned_string(s: String) -> usize {
    // Since we own the string, we can consume it freely
    let words: Vec<&str> = s.split_whitespace().collect();
    
    if words.is_empty() {
        return 0;
    }
    
    // Create a processed version
    let processed = words
        .into_iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<_>>()
        .join("_");
    
    processed.len()
}
```

## Simple Approach with Validation

```rust
fn process_owned_string(s: String) -> usize {
    // Validate and process the owned string
    if s.is_empty() {
        return 0;
    }
    
    // Trim whitespace and count non-whitespace characters
    s.trim()
     .chars()
     .filter(|c| !c.is_whitespace())
     .count()
}
```

## Memory-Efficient Version

```rust
fn process_owned_string(s: String) -> usize {
    // Since we own the string, we can move it through transformations
    let trimmed = s.trim().to_owned();
    
    if trimmed.is_empty() {
        return 0;
    }
    
    // Count alphabetic characters only
    trimmed.chars()
           .filter(|c| c.is_alphabetic())
           .count()
}
```

## Explanation

The process owned string function demonstrates fundamental ownership concepts in Rust:

1. **Parameter Ownership**: The function takes `String` by value, meaning ownership transfers from the caller to the function.

2. **Move Semantics**: Once passed to the function, the original `String` is moved and can no longer be used by the caller.

3. **String Transformation**: Since we own the string, we can:
   - Consume it in method chains
   - Modify it in place (if declared as `mut`)
   - Transform it into other types
   - Use methods that take `self` by value

4. **No Borrowing Needed**: Unlike functions that take `&str` or `&String`, this function doesn't need to worry about lifetimes or borrowing rules.

5. **Memory Management**: Rust automatically handles the memory cleanup when the function ends and the owned `String` goes out of scope.

6. **Use Cases**: Taking ownership is appropriate when:
   - You need to transform the string permanently
   - The caller doesn't need the string after the function call
   - You're building a processing pipeline that consumes inputs
   - You want to avoid lifetime complexity

This pattern is common in Rust for functions that consume their inputs to produce transformed outputs, ensuring clear ownership semantics and memory safety.