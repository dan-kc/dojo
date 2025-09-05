# Repeat Longer Owned

## Solution

```rust
pub fn repeat_longer_owned(s1: &str, s2: &str) -> String {
    let longer = if s1.len() >= s2.len() { s1 } else { s2 };
    format!("{}{}", longer, longer)
}
```

## Explanation

This solution demonstrates when lifetime annotations are not needed:

1. **Owned Return Type**: Returning `String` (owned data) means no lifetime annotations needed
2. **Local Decision**: The choice of which string is longer is made within the function
3. **String Creation**: Uses `format!` macro to efficiently create the repeated string

Key concepts:
- **No Lifetime Annotations**: When returning owned data, the output's lifetime is independent of inputs
- **Temporary References**: The `longer` variable holds a reference that's only used locally
- **Ownership Transfer**: The new `String` is created and ownership is transferred to the caller

Why no lifetime annotations are needed:
- The function creates new data (`String`) rather than returning a reference
- The returned `String` owns its data and doesn't depend on input lifetimes
- This is a common pattern when you need to combine or transform data

Alternative implementations:
```rust
// Using String methods
pub fn repeat_longer_owned(s1: &str, s2: &str) -> String {
    let longer = if s1.len() >= s2.len() { s1 } else { s2 };
    longer.repeat(2)
}

// Using push_str
pub fn repeat_longer_owned(s1: &str, s2: &str) -> String {
    let longer = if s1.len() >= s2.len() { s1 } else { s2 };
    let mut result = String::from(longer);
    result.push_str(longer);
    result
}
```