# Sliding Window Sums

## Solution

```rust
pub fn sliding_window_sums(numbers: Vec<i32>) -> Vec<i32> {
    numbers.windows(3)
        .map(|window| window.iter().sum())
        .collect()
}
```

## Explanation

This solution demonstrates the power of the `windows()` method for sliding window operations:

1. **Windows Iterator**: Creates overlapping slices of size 3
2. **Map**: Sums each window using `iter().sum()`
3. **Collect**: Gathers all window sums into a vector

Key concepts:
- **Overlapping vs Non-overlapping**: `windows()` creates overlapping slices, unlike `chunks()`
- **Slice References**: Each window is a slice reference `&[i32]`
- **Automatic Handling**: Returns empty iterator if the slice is shorter than window size
- **Efficiency**: No copying of data, works directly with slice references

Differences between `windows()` and `chunks()`:
- `windows(3)` on `[1,2,3,4,5]` → `[1,2,3]`, `[2,3,4]`, `[3,4,5]` (overlapping)
- `chunks(3)` on `[1,2,3,4,5]` → `[1,2,3]`, `[4,5]` (non-overlapping)

Common use cases for sliding windows:
- Moving averages in time series data
- Signal processing and filtering
- Pattern matching in sequences
- Local statistics computation

The `windows()` method is particularly efficient because it doesn't allocate new memory for each window, just provides different views into the same underlying data.