# iterators/parse_measurements

Reference implementation. Read after attempting the exercise.

```rust
pub fn parse_measurements(input: &str) -> Result<(Vec<i32>, i64), usize> {
    let values: Vec<i32> = input
        .lines()
        .enumerate()
        .filter(|(_, s)| !s.trim().is_empty())
        .map(|(i, s)| s.trim().parse::<i32>().map_err(|_| i + 1))
        .collect::<Result<_, _>>()?;
    let values: Vec<_> = values.into_iter().filter(|&n| n > 0).collect();
    let sum = values.iter().map(|&n| i64::from(n)).sum();
    Ok((values, sum))
}
```
