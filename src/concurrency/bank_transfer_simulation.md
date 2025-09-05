**Solution:**

```rust
fn bank_transfer_simulation(
    initial_balances: Vec<u64>,
    transfers: Vec<(usize, usize, u64)>,
) -> Vec<u64> {
    let accounts: Vec<_> = initial_balances
        .into_iter()
        .map(|balance| std::sync::Arc::new(std::sync::Mutex::new(balance)))
        .collect();
    
    let handles: Vec<_> = transfers
        .into_iter()
        .map(|(from, to, amount)| {
            let from_account = accounts[from].clone();
            let to_account = accounts[to].clone();
            
            std::thread::spawn(move || {
                // Always lock accounts in consistent order to prevent deadlock
                let (first, second, is_reversed) = if from < to {
                    (from_account, to_account, false)
                } else {
                    (to_account, from_account, true)
                };
                
                let mut first_guard = first.lock().unwrap();
                let mut second_guard = second.lock().unwrap();
                
                if is_reversed {
                    // second is from, first is to
                    if *second_guard >= amount {
                        *second_guard -= amount;
                        *first_guard += amount;
                    }
                } else {
                    // first is from, second is to
                    if *first_guard >= amount {
                        *first_guard -= amount;
                        *second_guard += amount;
                    }
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    accounts
        .into_iter()
        .map(|account| *account.lock().unwrap())
        .collect()
}
```

**Explanation:**

This solution demonstrates deadlock prevention in multi-mutex scenarios. Key concepts:

1. **Lock Ordering**: Always acquire locks in a consistent order (by account index) to prevent circular wait conditions
2. **Atomic Transfers**: Both balance updates happen while holding both locks, ensuring consistency
3. **Balance Validation**: Checks sufficient funds before transferring to prevent negative balances
4. **Money Conservation**: The total amount of money in the system remains constant

The lock ordering strategy is crucial for preventing deadlocks. If threads always acquire locks in the same order, circular dependencies cannot form. This is a fundamental technique in concurrent programming for handling multiple resources safely.