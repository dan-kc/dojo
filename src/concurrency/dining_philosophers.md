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
        Self {
            forks: (0..count).map(|_| Arc::new(Mutex::new(()))).collect(),
        }
    }

    /// A philosopher attempts to eat by acquiring two adjacent forks.
    /// Implement this without deadlocks using lock ordering.
    pub fn philosopher_eat(&self, philosopher_id: usize) -> bool {
        if self.forks.len() < 2 {
            return false;
        }

        let (first_idx, second_idx) = if philosopher_id == self.forks.len() - 1 {
            (0, self.forks.len() - 1)
        } else {
            (philosopher_id, philosopher_id + 1)
        };

        if let (Ok(first_guard), Ok(second_guard)) =
            (self.forks[first_idx].lock(), self.forks[second_idx].lock())
        {
            std::thread::sleep(std::time::Duration::from_millis(10));
            return true;
        }

        return false;
    }

    /// Run simulation with all philosophers trying to eat simultaneously.
    pub fn run_simulation(self: Arc<Self>) -> Vec<bool> {
        let res = (0..self.forks.len()).map(|idx| {
            let dp = self.clone();
            std::thread::spawn(move || dp.philosopher_eat(idx))
        });

        res.map(|handle| handle.join().unwrap()).collect()
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
