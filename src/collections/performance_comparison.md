# Performance Comparison Solution

## Implementation

```rust
pub fn performance_comparison(data_size: usize) -> (std::time::Duration, std::time::Duration) {
    use std::collections::HashMap;
    use std::time::Instant;
    
    if data_size == 0 {
        return (std::time::Duration::from_nanos(0), std::time::Duration::from_nanos(0));
    }
    
    // Prepare test data
    let data: Vec<(i32, String)> = (0..data_size)
        .map(|i| (i as i32, format!("value_{}", i)))
        .collect();
    
    // Setup HashMap
    let mut hashmap = HashMap::new();
    for (key, value) in &data {
        hashmap.insert(*key, value.clone());
    }
    
    // Setup Vec
    let vec_data: Vec<(i32, String)> = data.clone();
    
    // Test data for lookup (random keys from the dataset)
    let lookup_keys: Vec<i32> = (0..std::cmp::min(1000, data_size))
        .map(|i| (i * 13) as i32 % data_size as i32) // Pseudo-random distribution
        .collect();
    
    // Benchmark HashMap lookups
    let start = Instant::now();
    let mut hashmap_results = 0;
    for &key in &lookup_keys {
        if hashmap.contains_key(&key) {
            hashmap_results += 1;
        }
    }
    let hashmap_time = start.elapsed();
    
    // Benchmark Vec linear search
    let start = Instant::now();
    let mut vec_results = 0;
    for &key in &lookup_keys {
        if vec_data.iter().any(|(k, _)| *k == key) {
            vec_results += 1;
        }
    }
    let vec_time = start.elapsed();
    
    // Ensure both methods found the same results (sanity check)
    assert_eq!(hashmap_results, vec_results);
    
    (hashmap_time, vec_time)
}
```

## Educational Analysis Implementation

```rust
pub fn performance_comparison(data_size: usize) -> (std::time::Duration, std::time::Duration) {
    use std::collections::HashMap;
    use std::time::Instant;
    
    if data_size == 0 {
        return (std::time::Duration::from_nanos(0), std::time::Duration::from_nanos(0));
    }
    
    // Create test data
    let test_data: Vec<i32> = (0..data_size).map(|i| i as i32).collect();
    
    // Build HashMap
    let mut hashmap = HashMap::new();
    for &value in &test_data {
        hashmap.insert(value, value * 2);
    }
    
    // Clone Vec for linear search
    let vec_data = test_data.clone();
    
    // Generate lookup keys (mix of existing and non-existing)
    let lookup_count = 1000.min(data_size);
    let lookup_keys: Vec<i32> = (0..lookup_count)
        .map(|i| (i * 17 + 7) as i32 % (data_size as i32 + 100))
        .collect();
    
    // Benchmark HashMap O(1) average case
    let start = Instant::now();
    for &key in &lookup_keys {
        let _ = hashmap.get(&key);
    }
    let hashmap_duration = start.elapsed();
    
    // Benchmark Vec O(n) linear search
    let start = Instant::now();
    for &key in &lookup_keys {
        let _ = vec_data.iter().find(|&&x| x == key);
    }
    let vec_duration = start.elapsed();
    
    (hashmap_duration, vec_duration)
}
```

## Explanation

This solution compares HashMap vs Vec lookup performance:

1. **Data preparation**: Creates test datasets of the specified size
2. **Fair comparison**: Uses the same lookup keys for both data structures
3. **Realistic workload**: Mixes existing and non-existing keys
4. **Time measurement**: Uses `std::time::Instant` for precise timing

## Key Learning Points

- **Big O complexity**: HashMap O(1) average vs Vec O(n) linear search
- **Performance scaling**: HashMap advantage grows with dataset size
- **Memory vs speed**: HashMap uses more memory for faster lookups
- **Benchmarking methodology**: Fair comparison requires identical workloads

## Performance Characteristics

| Dataset Size | HashMap (Average) | Vec (Linear Search) |
|-------------|-------------------|---------------------|
| Small (< 100) | May be slower due to hashing overhead | Fast due to cache locality |
| Medium (100-1000) | Starts showing advantages | Performance degrades linearly |
| Large (> 1000) | Consistent O(1) performance | Noticeably slower O(n) |

## Use Cases for Each

**HashMap preferred for:**
- Frequent random access by key
- Large datasets with many lookups
- Key-based data organization
- Need for O(1) average lookup time

**Vec preferred for:**
- Sequential access patterns
- Small datasets
- Memory-constrained environments
- Simple data structures

## Rust Concepts Demonstrated

- Performance measurement with `std::time::Instant`
- Collection initialization and population patterns
- Iterator-based data processing
- Algorithmic complexity analysis
- Memory vs performance trade-offs
- Benchmarking methodology and fair comparisons