// Bank Transfer Simulation with Deadlock Prevention Practice
//
// Learning Objectives:
// - Handle multiple mutexes safely
// - Implement deadlock avoidance strategies
// - Ensure data consistency in concurrent operations
//
// cargo test --bin bank_transfer_simulation

/// Implement a bank account simulation where multiple threads perform
/// transfers between accounts. Ensure no money is lost or created.
/// Return the final balances of all accounts.
fn bank_transfer_simulation(
    initial_balances: Vec<u64>,
    transfers: Vec<(usize, usize, u64)>, // (from_account, to_account, amount)
) -> Vec<u64> {
    todo!("Implement bank transfer simulation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_transfer_simulation() {
        let initial = vec![1000, 2000, 3000];
        let transfers = vec![
            (0, 1, 100), // Account 0 -> Account 1: 100
            (1, 2, 200), // Account 1 -> Account 2: 200
            (2, 0, 50),  // Account 2 -> Account 0: 50
        ];
        
        let final_balances = bank_transfer_simulation(initial.clone(), transfers);
        
        // Check conservation of money
        let initial_total: u64 = initial.iter().sum();
        let final_total: u64 = final_balances.iter().sum();
        assert_eq!(initial_total, final_total);
        
        // Check specific balances
        assert_eq!(final_balances[0], 950);  // 1000 - 100 + 50
        assert_eq!(final_balances[1], 1900); // 2000 + 100 - 200
        assert_eq!(final_balances[2], 3150); // 3000 + 200 - 50
    }
}