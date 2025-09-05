# Sum of Squares of Even Numbers

## Solution

```rust
pub fn sum_squares_evens_iterator(numbers: &[i32]) -> i64 {
    numbers.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| (n as i64) * (n as i64))
        .sum()
}
```

## Explanation

This solution demonstrates the performance benefits of iterator chains in Rust:

1. **Iterator Chain**: We chain multiple iterator adaptors (`filter`, `map`) to create a pipeline that processes elements lazily
2. **Filter**: Selects only even numbers using the modulo operator
3. **Map**: Transforms each even number into its square, casting to `i64` to prevent overflow
4. **Sum**: Consumes the iterator and accumulates the results

The iterator approach is often more performant than loops because:
- The compiler can optimize iterator chains very effectively
- No intermediate allocations are needed
- The pipeline can be vectorized by the compiler
- Lazy evaluation means we only compute what we need

This approach also demonstrates Rust's zero-cost abstractions - the high-level iterator code compiles to machine code as efficient as (or better than) hand-written loops.