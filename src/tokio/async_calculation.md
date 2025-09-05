# Async Calculation

## Solution

```rust
use tokio::time::{sleep, Duration};

async fn async_calculation(a: i32, b: i32, delay_ms: u64) -> i32 {
    // Simulate async work (like network request or database query)
    sleep(Duration::from_millis(delay_ms)).await;
    
    // Perform the actual calculation
    a + b
}
```

## Alternative Implementation with More Complex Calculation

```rust
use tokio::time::{sleep, Duration};

async fn async_calculation(a: i32, b: i32, delay_ms: u64) -> i32 {
    // Simulate fetching data or performing I/O
    sleep(Duration::from_millis(delay_ms)).await;
    
    // Perform a more interesting calculation
    let sum = a + b;
    let product = a * b;
    let average = sum / 2;
    
    // Return a computed result
    sum + (product / 100).max(average)
}
```

## With Error Handling

```rust
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum CalculationError {
    Overflow,
    InvalidInput,
    NetworkTimeout,
}

async fn async_calculation_safe(
    a: i32, 
    b: i32, 
    delay_ms: u64
) -> Result<i32, CalculationError> {
    if delay_ms > 10_000 {
        return Err(CalculationError::NetworkTimeout);
    }
    
    if a == i32::MIN && b == i32::MIN {
        return Err(CalculationError::InvalidInput);
    }
    
    // Simulate async operation
    sleep(Duration::from_millis(delay_ms)).await;
    
    // Check for overflow
    match a.checked_add(b) {
        Some(result) => Ok(result),
        None => Err(CalculationError::Overflow),
    }
}
```

## Explanation

The async calculation function demonstrates combining computation with asynchronous operations:

1. **Async Simulation**: The delay simulates real-world async operations like network requests, database queries, or file I/O that would happen before or during computation.

2. **Simple Operation**: The basic implementation just adds two numbers, but the pattern applies to any computation that needs to happen asynchronously.

3. **Return Type**: The function returns the calculated value directly (not wrapped in a Future), which is automatically converted by the async system.

4. **Concurrency Benefits**: When multiple calculations run concurrently (using `tokio::join!`), their delays overlap, demonstrating the efficiency of async programming.

5. **Real-world Applications**: This pattern is common when you need to:
   - Fetch data from multiple sources before calculating
   - Perform calculations that depend on I/O operations
   - Process data with artificial delays (rate limiting, backoff)
   - Simulate expensive operations in tests

The key insight is that async functions can seamlessly mix I/O operations (awaited) with CPU-bound work (synchronous), making it easy to build responsive applications that don't block on I/O.