# concurrency/threaded_factorial

Reference implementation. Read after attempting the exercise.

```rust
fn threaded_factorial(n: u64) -> Result<u64, String> {
    let handle = std::thread::spawn(move || {
        let mut result = 1u64;
        for i in 1..=n {
            result = result.checked_mul(i).ok_or("overflow")?;
        }
        Ok::<u64, &str>(result)
    });

    match handle.join() {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(_)) => Err("Factorial calculation overflowed".to_string()),
        Err(_) => Err("Thread panicked during calculation".to_string()),
    }
}
```
