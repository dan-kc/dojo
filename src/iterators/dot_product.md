# Dot Product

## Solution

```rust
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}
```

## Explanation

This solution demonstrates efficient numerical computation using iterator combinators:

1. **Zip**: Pairs elements from both slices at corresponding indices
2. **Map**: Multiplies each pair of elements
3. **Sum**: Accumulates all products into the final result

Key concepts:
- **Parallel Iteration**: `zip()` efficiently iterates over two collections simultaneously
- **Automatic Length Handling**: `zip()` stops at the shorter slice's length, preventing index errors
- **Zero-cost Abstraction**: This high-level code compiles to efficient machine code comparable to hand-written loops
- **Functional Style**: The chain of transformations clearly expresses the mathematical operation

Performance benefits:
- **No Bounds Checking**: Iterator methods avoid redundant bounds checks
- **Vectorization**: The compiler can often vectorize this pattern for SIMD instructions
- **Cache Efficiency**: Sequential access pattern is cache-friendly
- **Lazy Evaluation**: No intermediate collections are created

This pattern is fundamental for numerical computing and demonstrates how Rust's iterators can express mathematical operations elegantly and efficiently.