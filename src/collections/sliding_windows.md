# collections/sliding_windows

Reference implementation. Read after attempting the exercise.

```rust
pub fn sliding_windows<T>(vec: Vec<T>, window_size: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    if window_size == 0 || vec.len() < window_size {
        return Vec::new();
    }

    vec.windows(window_size)
        .map(|window| window.to_vec())
        .collect()
}
```
