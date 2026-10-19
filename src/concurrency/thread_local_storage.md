# concurrency/thread_local_storage

Reference implementation. Read after attempting the exercise.

```rust
use std::cell::Cell;

use std::thread;

thread_local! {
    static THREAD_LOCAL_COUNTER: Cell<i32> = Cell::new(0);
}

pub fn increment_thread_local() {
    THREAD_LOCAL_COUNTER.with(|counter| {
        let current = counter.get();
        counter.set(current + 1);
    });
}

pub fn get_thread_local() -> i32 {
    THREAD_LOCAL_COUNTER.with(|counter| counter.get())
}
```
