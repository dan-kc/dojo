# Async Iterator Processing

## Solution

```rust
use tokio::time::{sleep, Duration};

async fn async_iterator_processing(items: Vec<i32>, delay_per_item_ms: u64) -> Vec<i32> {
    let mut results = Vec::with_capacity(items.len());
    
    for item in items {
        // Wait before processing each item
        sleep(Duration::from_millis(delay_per_item_ms)).await;
        
        // Process the item (example: double it)
        let processed = item * 2;
        results.push(processed);
    }
    
    results
}
```

## Alternative Implementation with Transformation

```rust
use tokio::time::{sleep, Duration};

async fn async_iterator_processing(items: Vec<i32>, delay_per_item_ms: u64) -> Vec<i32> {
    let mut results = Vec::new();
    
    for (index, item) in items.iter().enumerate() {
        // Simulate async processing delay
        sleep(Duration::from_millis(delay_per_item_ms)).await;
        
        // Apply different transformations based on value
        let processed = if *item < 0 {
            item.abs()  // Make negative numbers positive
        } else if *item == 0 {
            1           // Transform zero to one
        } else {
            item + index as i32  // Add index to positive numbers
        };
        
        results.push(processed);
    }
    
    results
}
```

## Stream-like Processing

```rust
use tokio::time::{sleep, Duration};

async fn async_iterator_processing(items: Vec<i32>, delay_per_item_ms: u64) -> Vec<i32> {
    use futures::stream::{self, StreamExt};
    
    // Convert to stream and process with delays
    stream::iter(items)
        .then(|item| async move {
            sleep(Duration::from_millis(delay_per_item_ms)).await;
            item * item  // Square each number
        })
        .collect()
        .await
}
```

## With Error Handling

```rust
use tokio::time::{sleep, Duration};

#[derive(Debug)]
enum ProcessingError {
    InvalidValue(i32),
    ProcessingFailed,
}

async fn async_iterator_processing_safe(
    items: Vec<i32>, 
    delay_per_item_ms: u64
) -> Result<Vec<i32>, ProcessingError> {
    let mut results = Vec::new();
    
    for item in items {
        // Validate input
        if item < -1000 || item > 1000 {
            return Err(ProcessingError::InvalidValue(item));
        }
        
        // Async processing delay
        sleep(Duration::from_millis(delay_per_item_ms)).await;
        
        // Simulate possible processing failure
        if item == 13 {  // Unlucky number
            return Err(ProcessingError::ProcessingFailed);
        }
        
        results.push(item.abs());
    }
    
    Ok(results)
}
```

## Explanation

The async iterator processing function demonstrates sequential async processing of collections:

1. **Sequential Processing**: Each item is processed one after another, with an async delay before processing each item.

2. **Async Loop**: The `for` loop with `await` inside creates a sequential async processing pattern where each iteration waits for the delay to complete.

3. **Memory Efficiency**: Results are collected incrementally rather than all at once, allowing for streaming-like behavior.

4. **Timing Behavior**: Total execution time is proportional to the number of items × delay per item, demonstrating sequential async processing.

5. **Transformation Patterns**: The function can apply various transformations to each item during the async processing.

6. **Stream Alternative**: The stream-based approach using `futures::stream` provides a more functional programming style for async iteration.

This pattern is useful for:
- Rate-limited API calls
- Processing items with delays between operations
- Sequential database operations
- Throttled file processing
- Any scenario where items must be processed one at a time with async delays