# First N Prime Numbers

## Solution

```rust
pub fn first_n_primes(n: usize) -> Vec<u32> {
    (2u32..)
        .filter(|&num| is_prime(num))
        .take(n)
        .collect()
}
```

## Explanation

This solution demonstrates lazy evaluation with iterators:

1. **Infinite Range**: `(2u32..)` creates an infinite iterator starting from 2
2. **Filter**: Only keeps numbers that pass the `is_prime` test
3. **Take**: Limits the output to exactly `n` elements, ensuring lazy evaluation
4. **Collect**: Gathers the results into a Vec

Key concepts:
- **Lazy Evaluation**: The iterator chain doesn't compute all primes upfront. It only generates primes as needed until `take(n)` is satisfied
- **Efficiency**: Once we have `n` primes, the iteration stops immediately - no wasted computation
- **Iterator Adaptors**: The combination of `filter` and `take` creates a powerful pattern for working with potentially infinite sequences

The `is_prime` helper uses an optimized algorithm that:
- Handles edge cases (numbers less than 2)
- Checks even numbers quickly
- Only checks odd divisors up to the square root of the number