**Solution:**

```rust
fn parallel_sum(numbers: Vec<i64>, num_threads: usize) -> i64 {
    if numbers.is_empty() || num_threads == 0 {
        return 0;
    }
    
    let chunk_size = (numbers.len() + num_threads - 1) / num_threads;
    let sum = std::sync::Arc::new(std::sync::Mutex::new(0i64));
    
    let handles: Vec<_> = numbers
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            let sum = sum.clone();
            std::thread::spawn(move || {
                let local_sum: i64 = chunk.iter().sum();
                let mut total = sum.lock().unwrap();
                *total += local_sum;
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    *sum.lock().unwrap()
}
```

**Explanation:**

This solution demonstrates parallel reduction with a shared accumulator. Key concepts:

1. **Work Distribution**: Divides the array into chunks for parallel processing
2. **Local Computation**: Each thread computes its local sum before acquiring the lock
3. **Minimal Lock Contention**: Lock is only held briefly to add the local sum to the total
4. **Scalability**: Performance improves with more threads up to the number of CPU cores

This pattern balances parallelism with synchronization overhead. Computing local sums reduces lock contention compared to incrementing the shared sum for each element. This technique is widely used in parallel algorithms for reductions and aggregations.