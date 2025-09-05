# BTree vs HashMap Performance Comparison Solution

## Implementation

```rust
fn performance_comparison(
    size: usize,
) -> (
    std::time::Duration, // BTreeMap insert
    std::time::Duration, // HashMap insert  
    std::time::Duration, // BTreeMap lookup
    std::time::Duration, // HashMap lookup
    std::time::Duration, // BTreeMap ordered iteration
    std::time::Duration, // HashMap unordered iteration
) {
    use std::collections::{BTreeMap, HashMap};
    use std::time::Instant;
    
    // Generate test data
    let data: Vec<(i32, i32)> = (0..size as i32)
        .map(|i| (i * 7 % (size as i32), i))
        .collect();
    
    // BTreeMap insertion
    let start = Instant::now();
    let mut btree = BTreeMap::new();
    for &(k, v) in &data {
        btree.insert(k, v);
    }
    let btree_insert = start.elapsed();
    
    // HashMap insertion
    let start = Instant::now();
    let mut hashmap = HashMap::new();
    for &(k, v) in &data {
        hashmap.insert(k, v);
    }
    let hash_insert = start.elapsed();
    
    // BTreeMap lookup
    let start = Instant::now();
    for i in 0..size as i32 {
        let _ = btree.get(&(i * 13 % (size as i32)));
    }
    let btree_lookup = start.elapsed();
    
    // HashMap lookup
    let start = Instant::now();
    for i in 0..size as i32 {
        let _ = hashmap.get(&(i * 13 % (size as i32)));
    }
    let hash_lookup = start.elapsed();
    
    // BTreeMap ordered iteration
    let start = Instant::now();
    let mut sum = 0i64;
    for (_, v) in &btree {
        sum += *v as i64;
    }
    let btree_iter = start.elapsed();
    
    // HashMap unordered iteration
    let start = Instant::now();
    let mut sum = 0i64;
    for (_, v) in &hashmap {
        sum += *v as i64;
    }
    let hash_iter = start.elapsed();
    
    (btree_insert, hash_insert, btree_lookup, hash_lookup, btree_iter, hash_iter)
}
```

## Explanation

This solution compares BTreeMap and HashMap performance:

1. **Test data generation**: Creates pseudo-random key-value pairs
2. **Insertion timing**: Measures time to insert all elements
3. **Lookup timing**: Measures random access performance
4. **Iteration timing**: Compares ordered vs unordered traversal
5. **Side effect prevention**: Uses sum to prevent optimization

## Key Learning Points

- **BTreeMap**: O(log n) operations, maintains order, cache-friendly iteration
- **HashMap**: O(1) average operations, no ordering, faster lookups
- **Trade-offs**: Order maintenance vs raw speed
- **Use cases**: BTreeMap for ordered data, HashMap for fast lookups

## Rust Concepts Demonstrated

- std::time::Instant for performance measurement
- Comparative benchmarking techniques
- Collection performance characteristics
- Memory layout implications
- Iterator performance differences