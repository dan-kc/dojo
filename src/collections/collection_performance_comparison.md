# Collection Performance Comparison Solution

## Implementation

```rust
pub fn collection_performance_comparison() -> (
    std::time::Duration, // Vec front insertion
    std::time::Duration, // VecDeque front insertion
    std::time::Duration, // Vec back insertion  
    std::time::Duration, // VecDeque back insertion
    std::time::Duration, // Vec random access
    std::time::Duration, // VecDeque random access
) {
    use std::time::Instant;
    
    let n = 10_000; // Number of operations for testing
    
    // Test Vec front insertion (expensive due to shifting)
    let start = Instant::now();
    let mut vec = std::vec::Vec::new();
    for i in 0..n {
        vec.insert(0, i);
    }
    let vec_front_duration = start.elapsed();
    
    // Test VecDeque front insertion (efficient)
    let start = Instant::now();
    let mut deque = std::collections::VecDeque::new();
    for i in 0..n {
        deque.push_front(i);
    }
    let deque_front_duration = start.elapsed();
    
    // Test Vec back insertion (efficient)
    let start = Instant::now();
    let mut vec = std::vec::Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    let vec_back_duration = start.elapsed();
    
    // Test VecDeque back insertion (efficient)
    let start = Instant::now();
    let mut deque = std::collections::VecDeque::new();
    for i in 0..n {
        deque.push_back(i);
    }
    let deque_back_duration = start.elapsed();
    
    // Setup collections for random access tests
    let vec: std::vec::Vec<i32> = (0..n).collect();
    let deque: std::collections::VecDeque<i32> = (0..n).collect();
    
    // Test Vec random access (efficient - O(1))
    let start = Instant::now();
    for i in 0..n {
        let _ = vec[i as usize % vec.len()];
    }
    let vec_random_duration = start.elapsed();
    
    // Test VecDeque random access (less efficient - O(1) but higher constant factor)
    let start = Instant::now();
    for i in 0..n {
        let _ = deque[i as usize % deque.len()];
    }
    let deque_random_duration = start.elapsed();
    
    (
        vec_front_duration,
        deque_front_duration,
        vec_back_duration,
        deque_back_duration,
        vec_random_duration,
        deque_random_duration,
    )
}
```

## Explanation

This solution benchmarks different collections for specific operations:

1. **Front insertion**: Vec requires O(n) shifting, while VecDeque is O(1) amortized
2. **Back insertion**: Both Vec and VecDeque are O(1) amortized
3. **Random access**: Vec has true O(1) access, VecDeque has O(1) with higher overhead
4. **Performance measurement**: Uses `Instant::now()` for timing operations

The benchmarks demonstrate the performance characteristics of each collection type.

## Key Learning Points

- **Vec front insertion**: Expensive due to element shifting (O(n) per operation)
- **VecDeque advantages**: Efficient operations at both ends
- **Random access trade-offs**: Vec is optimized for indexing, VecDeque has overhead
- **Choosing collections**: Performance characteristics guide collection selection

## Rust Concepts Demonstrated

- Performance measurement with `std::time::Instant`
- Collection operation complexity understanding
- Vec vs VecDeque trade-offs
- Benchmarking patterns in Rust
- Amortized complexity considerations