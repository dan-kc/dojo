# Timeout-Based Resource Management

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct TimeoutResourceManager {
    resources: Vec<Arc<Mutex<String>>>,
}

impl TimeoutResourceManager {
    pub fn new(resource_names: Vec<String>) -> Self {
        let resources = resource_names
            .into_iter()
            .map(|name| Arc::new(Mutex::new(name)))
            .collect();
        Self { resources }
    }

    pub fn acquire_with_timeout(&self, resource_id: usize, timeout: Duration) -> Option<String> {
        if resource_id >= self.resources.len() {
            return None;
        }

        let start_time = Instant::now();
        
        // Spin-wait with try_lock until timeout
        loop {
            if let Ok(lock) = self.resources[resource_id].try_lock() {
                return Some(lock.clone());
            }
            
            if start_time.elapsed() >= timeout {
                return None;
            }
            
            // Small delay to prevent excessive CPU usage
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn acquire_multiple_with_timeout(
        &self,
        resource_ids: Vec<usize>,
        timeout: Duration,
    ) -> Option<Vec<String>> {
        let start_time = Instant::now();
        
        // Try to acquire all resources until timeout
        loop {
            let mut acquired_locks = Vec::new();
            let mut all_acquired = true;
            
            // Try to acquire all resources with try_lock
            for &resource_id in &resource_ids {
                if resource_id >= self.resources.len() {
                    return None;
                }
                
                match self.resources[resource_id].try_lock() {
                    Ok(lock) => acquired_locks.push(lock),
                    Err(_) => {
                        all_acquired = false;
                        break;
                    }
                }
            }
            
            if all_acquired {
                // Successfully acquired all resources
                return Some(acquired_locks.into_iter().map(|lock| lock.clone()).collect());
            }
            
            // Check timeout
            if start_time.elapsed() >= timeout {
                return None;
            }
            
            // Release any acquired locks and retry
            drop(acquired_locks);
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    pub fn try_acquire_all(&self, resource_ids: Vec<usize>) -> Option<Vec<String>> {
        let mut acquired_locks = Vec::new();
        
        // Try to acquire all resources immediately
        for &resource_id in &resource_ids {
            if resource_id >= self.resources.len() {
                return None;
            }
            
            match self.resources[resource_id].try_lock() {
                Ok(lock) => acquired_locks.push(lock),
                Err(_) => {
                    // Failed to acquire a resource, abort
                    return None;
                }
            }
        }
        
        // Successfully acquired all resources
        Some(acquired_locks.into_iter().map(|lock| lock.clone()).collect())
    }
}
```

## Explanation

This solution implements timeout-based resource acquisition using **non-blocking locks** and **retry patterns**:

### Key Concepts Demonstrated:

1. **Non-blocking Resource Acquisition**:
   - Uses `try_lock()` instead of `lock()` to avoid indefinite blocking
   - Returns immediately with success or failure rather than waiting
   - Allows implementing custom timeout behavior

2. **Timeout Implementation**:
   - Uses `Instant::now()` and `elapsed()` to track timeout duration
   - Implements spin-wait loop with periodic checks
   - Balances responsiveness with CPU usage through small delays

3. **All-or-Nothing Semantics**:
   - For multiple resource acquisition, either all resources are acquired or none are
   - Prevents partial acquisitions that could lead to resource leaks or inconsistent states
   - Uses RAII (automatic drop) to release locks on failure

4. **Graceful Degradation**:
   - Functions return `Option` to indicate success or timeout
   - Allows callers to handle resource contention appropriately
   - No panics or indefinite blocking

### Resource Contention Handling:

1. **Single Resource**: Retries acquisition until timeout, preventing indefinite waits
2. **Multiple Resources**: Uses optimistic approach - try all, retry if any fail  
3. **Non-blocking**: Immediate failure if resources aren't available (try_acquire_all)

### Performance Considerations:

- **Spin-wait approach**: Trades some CPU usage for quick response times
- **Small delays**: 1ms sleep prevents excessive CPU consumption
- **Early termination**: Timeout checks prevent unnecessary retries

### Use Cases:

This pattern is valuable in systems where:
- Resource availability is unpredictable
- Blocking indefinitely is unacceptable  
- Graceful fallback behavior is preferred over failure
- Resource contention needs to be managed proactively

The timeout-based approach provides predictable behavior and prevents thread starvation, making it suitable for real-time systems or user-facing applications where responsiveness is critical.