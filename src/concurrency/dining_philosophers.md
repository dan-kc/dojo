# Dining Philosophers Deadlock Prevention

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub struct DiningPhilosophers {
    forks: Vec<Arc<Mutex<()>>>,
}

impl DiningPhilosophers {
    pub fn new(count: usize) -> Self {
        let forks = (0..count).map(|_| Arc::new(Mutex::new(()))).collect();
        Self { forks }
    }

    pub fn philosopher_eat(&self, philosopher_id: usize) -> bool {
        let count = self.forks.len();
        let left_fork = philosopher_id;
        let right_fork = (philosopher_id + 1) % count;
        
        // Prevent deadlock by ordering lock acquisition consistently
        // Always acquire lower-numbered fork first
        let (first_fork, second_fork) = if left_fork < right_fork {
            (left_fork, right_fork)
        } else {
            (right_fork, left_fork)
        };
        
        // Try to acquire both forks with consistent ordering
        let _first_lock = match self.forks[first_fork].try_lock() {
            Ok(lock) => lock,
            Err(_) => return false, // Fork not available
        };
        
        let _second_lock = match self.forks[second_fork].try_lock() {
            Ok(lock) => lock,
            Err(_) => return false, // Fork not available
        };
        
        // Both forks acquired - philosopher can eat
        // Simulate eating time
        thread::sleep(std::time::Duration::from_millis(1));
        
        true
    }

    pub fn run_simulation(&self) -> Vec<bool> {
        let count = self.forks.len();
        let handles: Vec<_> = (0..count)
            .map(|i| {
                let philosophers_ref = self;
                thread::spawn(move || philosophers_ref.philosopher_eat(i))
            })
            .collect();
        
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    }
}
```

## Explanation

This solution implements the classic dining philosophers problem with deadlock prevention using **lock ordering**:

### Key Concepts Demonstrated:

1. **Lock Ordering Strategy**: 
   - Always acquire locks in a consistent order (lower-numbered fork first)
   - This breaks the circular wait condition that causes deadlocks
   - Even if all philosophers try to eat simultaneously, they won't deadlock

2. **Arc and Mutex Usage**:
   - `Arc<Mutex<()>>` allows sharing fork locks across multiple threads
   - The unit type `()` is used since we only care about mutual exclusion, not data

3. **Deadlock Prevention**:
   - Instead of each philosopher taking left fork then right fork, we enforce a global ordering
   - This prevents the circular dependency that causes deadlock in the naive approach

4. **Non-blocking Acquisition**:
   - Uses `try_lock()` to avoid blocking indefinitely
   - Returns `false` if forks aren't immediately available

### The Deadlock Problem:
Without lock ordering, if all philosophers simultaneously pick up their left fork and then try to pick up their right fork, they would wait forever in a circular dependency.

### Our Solution:
By always acquiring the lower-numbered fork first, we create a partial ordering of resources that prevents circular waiting. This is a fundamental technique in concurrent programming for deadlock prevention.

The simulation spawns threads for each philosopher attempting to eat simultaneously, demonstrating that the solution handles concurrent access without deadlocks.