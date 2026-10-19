# iterators/dot_product

Reference implementation. Read after attempting the exercise.

```rust
pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}
```
