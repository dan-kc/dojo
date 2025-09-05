# Elements Above Average

## Solution

```rust
pub fn elements_above_average(data: &[Vec<i32>]) -> Vec<i32> {
    data.iter()
        .flat_map(|inner| {
            if inner.is_empty() {
                return vec![];
            }
            
            let sum: i32 = inner.iter().sum();
            let avg = sum as f64 / inner.len() as f64;
            
            inner.iter()
                .filter(|&&x| x as f64 > avg)
                .copied()
                .collect::<Vec<_>>()
        })
        .collect()
}
```

## Explanation

This solution efficiently processes nested data structures:

1. **Outer Iteration**: Iterates over each inner vector
2. **Average Calculation**: Computes the average for each inner vector
3. **Filtering**: Selects elements greater than their vector's average
4. **Flat Map**: Flattens the filtered results from all inner vectors into a single collection

Key concepts:
- **Flat Map**: Combines `map` and `flatten` operations, perfect for nested structures
- **Empty Handling**: Guards against division by zero with empty vectors
- **Type Conversions**: Careful conversion between `i32` and `f64` for accurate comparisons
- **Copied**: Converts `&i32` references to `i32` values efficiently

The algorithm:
1. For each inner vector, calculate its average
2. Filter elements greater than that average
3. Flatten all results into a single vector

This pattern is useful for:
- Statistical analysis across grouped data
- Finding outliers within categories
- Processing hierarchical data structures