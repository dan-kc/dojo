# iterators/chunk_averages

Reference implementation. Read after attempting the exercise.

```rust
pub fn chunk_averages(numbers: &[f64], chunk_size: usize) -> Vec<f64> {
    if chunk_size == 0 {
        return vec![];
    }

    numbers
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
        .collect()
}
```
