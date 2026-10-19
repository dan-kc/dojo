# concurrency/bank_transfer_simulation

Reference implementation. Read after attempting the exercise.

```rust
pub struct Accounts {
    balances: Vec<std::sync::Mutex<u64>>,
}
impl Accounts {
    pub fn new(balances: Vec<u64>) -> Self {
        Self {
            balances: balances.into_iter().map(std::sync::Mutex::new).collect(),
        }
    }
    pub fn balance(&self, id: usize) -> Option<u64> {
        self.balances.get(id).map(|b| *b.lock().unwrap())
    }
    pub fn transfer(&self, from: usize, to: usize, amount: u64) -> bool {
        if from >= self.balances.len() || to >= self.balances.len() {
            return false;
        }
        if from == to {
            return true;
        }
        let low = from.min(to);
        let high = from.max(to);
        let mut a = self.balances[low].lock().unwrap();
        let mut b = self.balances[high].lock().unwrap();
        let (src, dst) = if from == low {
            (&mut *a, &mut *b)
        } else {
            (&mut *b, &mut *a)
        };
        if *src < amount {
            return false;
        }
        let Some(new) = dst.checked_add(amount) else {
            return false;
        };
        *src -= amount;
        *dst = new;
        true
    }
}
```
