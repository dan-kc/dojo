# Async Closure Usage

## Solution

```rust
async fn async_closure_usage<F, Fut>(items: Vec<i32>, async_processor: F) -> Vec<i32>
where
    F: Fn(i32) -> Fut,
    Fut: std::future::Future<Output = i32>,
{
    let mut results = Vec::with_capacity(items.len());
    
    for item in items {
        // Call the async processor and await the result
        let processed = async_processor(item).await;
        results.push(processed);
    }
    
    results
}
```

## Alternative Implementation with Concurrent Processing

```rust
use futures::future::join_all;

async fn async_closure_usage<F, Fut>(items: Vec<i32>, async_processor: F) -> Vec<i32>
where
    F: Fn(i32) -> Fut + Copy,
    Fut: std::future::Future<Output = i32>,
{
    // Process all items concurrently
    let futures: Vec<_> = items.into_iter()
        .map(|item| async_processor(item))
        .collect();
    
    join_all(futures).await
}
```

## With Error Handling

```rust
async fn async_closure_usage_safe<F, Fut, E>(
    items: Vec<i32>, 
    async_processor: F
) -> Result<Vec<i32>, E>
where
    F: Fn(i32) -> Fut,
    Fut: std::future::Future<Output = Result<i32, E>>,
{
    let mut results = Vec::with_capacity(items.len());
    
    for item in items {
        match async_processor(item).await {
            Ok(processed) => results.push(processed),
            Err(e) => return Err(e),
        }
    }
    
    Ok(results)
}
```

## Stream-Based Implementation

```rust
use futures::stream::{self, StreamExt};

async fn async_closure_usage_stream<F, Fut>(
    items: Vec<i32>, 
    async_processor: F
) -> Vec<i32>
where
    F: Fn(i32) -> Fut,
    Fut: std::future::Future<Output = i32>,
{
    stream::iter(items)
        .then(async_processor)
        .collect()
        .await
}
```

## Advanced with Custom Future

```rust
use std::pin::Pin;
use std::future::Future;

// More flexible trait bounds
async fn async_closure_usage_advanced<F>(
    items: Vec<i32>, 
    async_processor: F
) -> Vec<i32>
where
    F: for<'a> Fn(i32) -> Pin<Box<dyn Future<Output = i32> + Send + 'a>>,
{
    let mut results = Vec::new();
    
    for item in items {
        let future = async_processor(item);
        let result = future.await;
        results.push(result);
    }
    
    results
}
```

## Explanation

The async closure usage function demonstrates working with higher-order async functions:

1. **Generic Parameters**: The function is generic over both the closure `F` and the future type `Fut` that the closure returns.

2. **Trait Bounds**: 
   - `F: Fn(i32) -> Fut` means F is a function that takes an i32 and returns a Future
   - `Fut: std::future::Future<Output = i32>` means the Future resolves to an i32

3. **Sequential Processing**: The basic implementation processes items one at a time, awaiting each async closure call.

4. **Concurrent Alternative**: The `join_all` version processes all items concurrently, which can be much faster but may consume more resources.

5. **Flexibility**: This pattern allows users to provide their own async processing logic while the function handles the iteration and result collection.

6. **Real-world Applications**:
   - Processing items with async transformations (API calls, database lookups)
   - Applying async validation or enrichment functions
   - Parallel processing with custom async logic
   - Building async processing pipelines

The key insight is that async closures allow you to pass custom async behavior to higher-order functions, enabling flexible and reusable async processing patterns.