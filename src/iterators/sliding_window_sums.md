# iterators/sliding_window_sums

Reference implementation. Read after attempting the exercise.

```rust
pub fn sliding_window_sums(numbers: &[i32], width: std::num::NonZeroUsize) -> Vec<i64> {
    let k = width.get();
    if k > numbers.len() {
        return Vec::new();
    }
    let mut sum: i64 = numbers[..k].iter().map(|&n| i64::from(n)).sum();
    let mut out = vec![sum];
    for i in k..numbers.len() {
        sum += i64::from(numbers[i]) - i64::from(numbers[i - k]);
        out.push(sum);
    }
    out
}
```
