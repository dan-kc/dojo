**Solution:**

```rust
fn threaded_factorial(n: u64) -> Result<u64, String> {
    let handle = std::thread::spawn(move || {
        let mut result = 1u64;
        for i in 1..=n {
            result = result.checked_mul(i)?;
        }
        Ok(result)
    });
    
    match handle.join() {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(_)) => Err("Factorial calculation overflowed".to_string()),
        Err(_) => Err("Thread panicked during calculation".to_string()),
    }
}
```

**Explanation:**

This solution demonstrates error handling with threads in Rust. Key concepts:

1. **Checked Arithmetic**: Uses `checked_mul` to detect overflow without panicking
2. **Double Result Handling**: The thread returns `Result<u64, ()>`, and `join()` also returns a Result indicating if the thread panicked
3. **Error Propagation**: The `?` operator inside the thread propagates overflow errors
4. **Pattern Matching**: Handles both thread panic (outer Err) and calculation failure (inner Err)

The factorial of 25 exceeds u64::MAX, demonstrating the overflow handling. This pattern is useful for running potentially failing computations in separate threads while maintaining error safety.