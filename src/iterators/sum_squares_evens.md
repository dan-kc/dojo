# iterators/sum_squares_evens

Reference implementation. Read after attempting the exercise.

```rust
pub fn sum_squares_evens_iterator(numbers: &[i32]) -> i128 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| {
            let n = i128::from(n);
            n * n
        })
        .sum()
}
```
