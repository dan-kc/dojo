# iterators/elements_above_average

Reference implementation. Read after attempting the exercise.

```rust
pub fn elements_above_average(data: &[Vec<i32>]) -> Vec<i32> {
    data.iter()
        .flat_map(|inner| {
            if inner.is_empty() {
                return vec![];
            }

            let sum: i128 = inner.iter().map(|&x| i128::from(x)).sum();

            inner
                .iter()
                .filter(|&&x| i128::from(x) * inner.len() as i128 > sum)
                .copied()
                .collect::<Vec<_>>()
        })
        .collect()
}
```
