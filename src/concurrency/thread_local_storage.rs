// cargo test --lib concurrency::thread_local_storage::

//! Each std::thread owns an independent counter initially zero. Increment changes only the calling std::thread's counter; get reads only that counter. Calls stay within i32 range.

/// Thread-local storage example that demonstrates non-Send/non-Sync patterns
// Declare the per-thread storage here.

pub fn increment_thread_local() {
    todo!()
}

pub fn get_thread_local() -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::thread;

    #[test]
    fn test_thread_local_storage() {
        let handles: Vec<_> = (0..3)
            .map(|thread_id| {
                thread::spawn(move || {
                    // Each thread has its own counter
                    for _ in 0..10 {
                        increment_thread_local();
                    }
                    (thread_id, get_thread_local())
                })
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // Each thread should have count 10
        for (thread_id, count) in results {
            assert_eq!(count, 10, "Thread {} should have count 10", thread_id);
        }

        // Main thread should still have count 0
        assert_eq!(get_thread_local(), 0);
    }
}
