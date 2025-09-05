# Chunk Averages

## Solution

```rust
pub fn chunk_averages(numbers: &[f64], chunk_size: usize) -> Vec<f64> {
    if chunk_size == 0 {
        return vec![];
    }

    numbers.chunks(chunk_size)
        .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
        .collect()
}
```

## Explanation

This solution demonstrates efficient batch processing using iterator methods:

1. **Edge Case Handling**: Returns empty vector for zero chunk size to avoid panic
2. **Chunks Iterator**: `chunks(chunk_size)` divides the slice into non-overlapping chunks
3. **Average Calculation**: For each chunk, sum all elements and divide by chunk length
4. **Automatic Handling**: The last chunk may be smaller than `chunk_size`, which is handled automatically

Key concepts:

- **Batch Processing**: `chunks()` is perfect for processing data in fixed-size batches
- **Efficiency**: No intermediate allocations, processes chunks lazily
- **Flexibility**: The last chunk can be smaller, making this method practical for real-world data
- **Type Conversion**: Careful conversion of `chunk.len()` to `f64` for floating-point division

The `chunks()` method is particularly useful for:

- Processing large datasets in manageable pieces
- Parallel processing preparation
- Memory-efficient streaming operations

