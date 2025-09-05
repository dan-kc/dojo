// Deadlock-Free Bank Transfer Practice
//
// Learning Objectives:
// - Implement bank transfers that prevent deadlocks
// - Use consistent ordering of account locks
// - Handle concurrent transfers safely
//
// Run with: cargo test --bin deadlock_free_bank

use std::sync::{Arc, Mutex};
use std::thread;

/// Implement a bank transfer system that prevents deadlocks when
/// transferring between accounts. Use consistent ordering of account locks.
pub struct DeadlockFreeBank {
    accounts: Vec<Arc<Mutex<u64>>>,
}

impl DeadlockFreeBank {
    pub fn new(initial_balances: Vec<u64>) -> Self {
        todo!("Implement new")
    }

    /// Transfer money between accounts without deadlocks.
    /// Always acquire locks in ascending order of account ID.
    pub fn transfer(&self, from: usize, to: usize, amount: u64) -> bool {
        todo!("Implement deadlock-free transfer")
    }

    /// Get balance of an account.
    pub fn balance(&self, account_id: usize) -> u64 {
        todo!("Implement balance")
    }

    /// Attempt multiple concurrent transfers without deadlocks.
    pub fn concurrent_transfers(&self, transfers: Vec<(usize, usize, u64)>) -> Vec<bool> {
        todo!("Implement concurrent transfers")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deadlock_free_bank() {
        let bank = DeadlockFreeBank::new(vec![1000, 2000, 3000]);
        
        // Test single transfer
        assert!(bank.transfer(0, 1, 100));
        assert_eq!(bank.balance(0), 900);
        assert_eq!(bank.balance(1), 2100);
        
        // Test concurrent transfers
        let transfers = vec![
            (1, 2, 200),
            (2, 0, 150),
            (0, 1, 50),
        ];
        let results = bank.concurrent_transfers(transfers);
        
        // All transfers should succeed without deadlock
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|&success| success));
        
        // Verify total money conservation
        let total = bank.balance(0) + bank.balance(1) + bank.balance(2);
        assert_eq!(total, 6000);
    }

    #[test]
    fn test_no_deadlock_under_stress() {
        let bank = Arc::new(DeadlockFreeBank::new(vec![1000; 10]));
        
        // Create many concurrent transfers that could cause deadlocks
        let mut handles = Vec::new();
        
        for _ in 0..50 {
            let bank_clone = bank.clone();
            let handle = thread::spawn(move || {
                for i in 0..20 {
                    let from = i % 10;
                    let to = (i + 1) % 10;
                    bank_clone.transfer(from, to, 10);
                }
            });
            handles.push(handle);
        }
        
        // All threads should complete without deadlock
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify total balance is conserved
        let total: u64 = (0..10).map(|i| bank.balance(i)).sum();
        assert_eq!(total, 10000);
    }
}