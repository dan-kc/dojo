# smart_pointers/cell_counter

Reference implementation. Read after attempting the exercise.

```rust
use std::cell::Cell;

pub struct CellCounter {
    count: Cell<i32>,
}

impl CellCounter {
    pub fn new(initial: i32) -> Self {
        CellCounter {
            count: Cell::new(initial),
        }
    }

    pub fn get(&self) -> i32 {
        self.count.get()
    }

    pub fn set(&self, value: i32) {
        self.count.set(value);
    }

    pub fn increment(&self) -> i32 {
        let current = self.count.get();
        self.count.set(current + 1);
        current
    }

    pub fn add(&self, amount: i32) -> i32 {
        let new_value = self.count.get() + amount;
        self.count.set(new_value);
        new_value
    }
}
```
