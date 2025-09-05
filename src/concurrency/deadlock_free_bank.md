# Deadlock-Free Bank Transfer

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub struct DeadlockFreeBank {
    accounts: Vec<Arc<Mutex<u64>>>,
}

impl DeadlockFreeBank {
    pub fn new(initial_balances: Vec<u64>) -> Self {
        let accounts = initial_balances
            .into_iter()
            .map(|balance| Arc::new(Mutex::new(balance)))
            .collect();
        Self { accounts }
    }

    pub fn transfer(&self, from: usize, to: usize, amount: u64) -> bool {
        if from == to || from >= self.accounts.len() || to >= self.accounts.len() {
            return false;
        }

        // Prevent deadlock by always acquiring locks in ascending order of account ID
        let (first_id, second_id) = if from < to {
            (from, to)
        } else {
            (to, from)
        };

        let first_lock = self.accounts[first_id].lock().unwrap();
        let second_lock = self.accounts[second_id].lock().unwrap();

        // Now safely access the accounts in the original order
        let (from_balance, to_balance) = if from < to {
            (&first_lock, &second_lock)
        } else {
            (&second_lock, &first_lock)
        };

        // Check if from account has sufficient funds
        if *from_balance < amount {
            return false;
        }

        // Perform the transfer by directly modifying through the mutex guards
        // We need to get mutable references, so we'll restructure this
        drop(first_lock);
        drop(second_lock);

        // Re-acquire in order for mutable access
        let mut first_lock = self.accounts[first_id].lock().unwrap();
        let mut second_lock = self.accounts[second_id].lock().unwrap();

        let (from_mut, to_mut) = if from < to {
            (&mut *first_lock, &mut *second_lock)
        } else {
            (&mut *second_lock, &mut *first_lock)
        };

        if *from_mut < amount {
            return false;
        }

        *from_mut -= amount;
        *to_mut += amount;
        
        true
    }

    pub fn balance(&self, account_id: usize) -> u64 {
        if account_id >= self.accounts.len() {
            return 0;
        }
        *self.accounts[account_id].lock().unwrap()
    }

    pub fn concurrent_transfers(&self, transfers: Vec<(usize, usize, u64)>) -> Vec<bool> {
        let handles: Vec<_> = transfers
            .into_iter()
            .map(|(from, to, amount)| {
                let bank_ref = self;
                thread::spawn(move || bank_ref.transfer(from, to, amount))
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

This solution implements deadlock-free bank transfers using **consistent lock ordering**:

### Key Concepts Demonstrated:

1. **Deadlock Prevention Through Lock Ordering**:
   - Always acquire account locks in ascending order of account ID
   - This creates a total ordering that prevents circular wait conditions
   - Even with concurrent transfers between the same accounts in opposite directions, no deadlock occurs

2. **Account Isolation**:
   - Each account is protected by its own `Arc<Mutex<u64>>`
   - Allows fine-grained locking - only the accounts involved in a transfer are locked
   - Better concurrency than a single global lock

3. **Atomic Transfer Operations**:
   - Both account balances are modified while holding both locks
   - Ensures transfer is atomic - either both accounts are updated or neither is
   - Prevents inconsistent states during concurrent operations

4. **Safe Concurrent Access**:
   - Multiple threads can perform transfers simultaneously without interfering
   - The lock ordering prevents deadlocks even under high contention
   - Sufficient funds check is performed while holding the lock

### The Deadlock Problem:
Without consistent ordering, two threads could deadlock:
- Thread A: locks account 1, tries to lock account 2  
- Thread B: locks account 2, tries to lock account 1

### Our Solution:
By always acquiring locks in ascending order of account ID (lower ID first), we eliminate the possibility of circular waiting. This is a classic technique in database systems and concurrent programming.

### Performance Considerations:
- Fine-grained locking allows multiple non-conflicting transfers to proceed simultaneously
- Lock ordering adds minimal overhead but eliminates deadlock risk
- The solution maintains ACID properties for financial transactions

This approach demonstrates how careful design of lock acquisition order can prevent deadlocks while maintaining good concurrency performance.