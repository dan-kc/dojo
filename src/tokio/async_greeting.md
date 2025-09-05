# Async Greeting

## Solution

```rust
use tokio::time::{sleep, Duration};

async fn async_greeting(name: &str, delay_ms: u64) -> String {
    // Wait for the specified duration
    sleep(Duration::from_millis(delay_ms)).await;
    
    // Return a greeting message
    format!("Hello, {}!", name)
}
```

## Alternative Implementation

```rust
use tokio::time::{sleep, Duration};

async fn async_greeting(name: &str, delay_ms: u64) -> String {
    // Perform async operation first
    sleep(Duration::from_millis(delay_ms)).await;
    
    // Create a more elaborate greeting
    if name.is_empty() {
        "Hello, anonymous user!".to_string()
    } else {
        format!("Greetings, {}! Welcome to the async world!", name)
    }
}
```

## Explanation

The async greeting function demonstrates basic async/await patterns:

1. **Async Function Declaration**: The `async fn` keyword makes the function return a `Future` that can be awaited.

2. **Awaiting Sleep**: `tokio::time::sleep()` returns a future that completes after the specified duration. The `.await` keyword suspends execution until the sleep completes.

3. **Return Value**: The function returns a `String` directly (not wrapped in a Future), which is automatically converted to `Future<Output = String>`.

4. **Concurrency**: When multiple async greetings are run concurrently (using `tokio::join!`), they can execute their delays simultaneously rather than sequentially.

5. **Zero-Cost Abstractions**: The async/await syntax compiles down to efficient state machines that don't block OS threads during waits.

This pattern is fundamental to async Rust programming and demonstrates how async functions can perform I/O operations (like delays) without blocking the thread.