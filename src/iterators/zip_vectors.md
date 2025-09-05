# Zip Vectors

## Solution

```rust
pub fn zip_vectors<T, U>(first: Vec<T>, second: Vec<U>) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    first.into_iter()
        .zip(second.into_iter())
        .collect()
}
```

## Explanation

This solution demonstrates the power of the `zip()` iterator adaptor:

1. **Into Iterator**: Converts both vectors into iterators that consume the original values
2. **Zip Operation**: Pairs elements at corresponding positions
3. **Automatic Length Handling**: Stops at the shorter vector's length
4. **Collection**: Gathers the pairs into a vector of tuples

Key concepts:
- **Generic Types**: Works with any types T and U that implement Clone
- **Move Semantics**: Uses `into_iter()` to take ownership and avoid unnecessary cloning
- **Safety**: Automatically handles mismatched lengths without panicking
- **Efficiency**: No manual indexing or bounds checking needed

Alternative approaches:
- Using `iter()` with `cloned()` if you need to keep the original vectors
- Using `iter().zip()` with references if you don't need ownership
- Manual implementation with indices (less idiomatic and more error-prone)

The `zip()` pattern is fundamental for:
- Parallel data processing
- Creating associations between related collections
- Implementing mathematical operations on vectors