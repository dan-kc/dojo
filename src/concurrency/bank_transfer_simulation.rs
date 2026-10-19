// cargo test --lib concurrency::bank_transfer_simulation::

//! Implement shared accounts with atomic transfers. Lock only the two affected accounts in a consistent order.
//! transfer returns false for an invalid account, insufficient funds, or destination overflow; failed transfers change nothing.
//! A self-transfer succeeds for a valid account without changing its balance. Other successful transfers conserve money.
//! balance returns None for an invalid ID. Concurrent opposite-direction transfers must finish without deadlock.

pub struct Accounts {
    balances: Vec<std::sync::Mutex<u64>>,
}
impl Accounts {
    pub fn new(balances: Vec<u64>) -> Self {
        todo!()
    }
    pub fn balance(&self, id: usize) -> Option<u64> {
        todo!()
    }
    pub fn transfer(&self, from: usize, to: usize, amount: u64) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{mpsc, Arc};
    use std::time::Duration;
    #[test]
    fn failures_are_atomic() {
        let a = Accounts::new(vec![10, u64::MAX]);
        assert!(!a.transfer(0, 1, 1));
        assert!(!a.transfer(0, 1, 11));
        assert!(!a.transfer(9, 0, 1));
        assert!(a.transfer(0, 0, 99));
        assert_eq!(a.balance(0), Some(10));
        assert_eq!(a.balance(1), Some(u64::MAX));
        assert_eq!(a.balance(2), None);
    }
    #[test]
    fn concurrent_opposite_transfers() {
        let a = Arc::new(Accounts::new(vec![1000, 1000]));
        let (tx, rx) = mpsc::channel();
        let mut workers = vec![];
        for from in 0..2 {
            let a = a.clone();
            let tx = tx.clone();
            workers.push(std::thread::spawn(move || {
                for _ in 0..500 {
                    assert!(a.transfer(from, 1 - from, 1));
                }
                tx.send(()).unwrap();
            }));
        }
        for _ in 0..2 {
            rx.recv_timeout(Duration::from_secs(5)).expect("deadlock");
        }
        for w in workers {
            w.join().unwrap();
        }
        assert_eq!(a.balance(0), Some(1000));
        assert_eq!(a.balance(1), Some(1000));
    }
}
