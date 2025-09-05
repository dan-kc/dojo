# Sequential Async Operations

## Solution

```rust
use tokio::time::{sleep, Duration};

async fn sequential_async_operations(steps: Vec<(&str, u64)>) -> String {
    let mut result = String::new();
    
    for (step_name, delay_ms) in steps {
        // Wait for the specified delay
        sleep(Duration::from_millis(delay_ms)).await;
        
        // Add this step to the result
        if !result.is_empty() {
            result.push_str(" -> ");
        }
        result.push_str(step_name);
    }
    
    result
}
```

## Alternative Implementation with Progress Tracking

```rust
use tokio::time::{sleep, Duration};

async fn sequential_async_operations(steps: Vec<(&str, u64)>) -> String {
    if steps.is_empty() {
        return "No operations to perform".to_string();
    }
    
    let mut result = String::from("Starting operations: ");
    
    for (index, (step_name, delay_ms)) in steps.iter().enumerate() {
        // Add step information
        result.push_str(&format!("[{}/{}] {}", index + 1, steps.len(), step_name));
        
        // Perform async delay
        sleep(Duration::from_millis(*delay_ms)).await;
        
        // Mark completion
        result.push_str(" ✓");
        
        if index < steps.len() - 1 {
            result.push_str(" | ");
        }
    }
    
    result.push_str(" | All operations completed!");
    result
}
```

## With Error Simulation

```rust
use tokio::time::{sleep, Duration};

#[derive(Debug)]
struct StepError {
    step: String,
    reason: String,
}

async fn sequential_async_operations_with_errors(
    steps: Vec<(&str, u64)>
) -> Result<String, StepError> {
    let mut result = String::new();
    
    for (step_name, delay_ms) in steps {
        // Simulate potential failure for demonstration
        if step_name.contains("fail") {
            return Err(StepError {
                step: step_name.to_string(),
                reason: "Simulated failure".to_string(),
            });
        }
        
        // Wait asynchronously
        sleep(Duration::from_millis(delay_ms)).await;
        
        // Accumulate results
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&format!("[{}]", step_name));
    }
    
    Ok(result)
}
```

## Explanation

The sequential async operations function demonstrates async iteration and accumulation patterns:

1. **Sequential Processing**: Each step is processed one after another, with the `await` ensuring the delay completes before moving to the next step.

2. **Async Loop**: The `for` loop with `await` inside creates sequential async processing - each iteration waits for the previous to complete.

3. **State Accumulation**: The result string is built incrementally as each async operation completes.

4. **Timing Behavior**: Total execution time is the sum of all delays, demonstrating sequential (not concurrent) execution.

5. **Error Handling**: The alternative version shows how to propagate errors through sequential async operations using `Result` types.

This pattern is useful when operations must be performed in order, such as initialization sequences, pipeline processing, or when later operations depend on results from earlier ones.