// cargo test --lib concurrency::parallel_map::

//! Transform each item exactly once, using at most workers scoped worker threads.
//! Return results in input order and join every worker before returning. Borrowed inputs and closures are allowed;
//! neither items nor results need Clone. Empty input returns empty output. A worker panic must propagate
//! after all workers are joined. Use actual threads; no simulated pool or sequential fallback for nonempty multiworker work.

pub fn parallel_map<T: Send, R: Send, F: Fn(T) -> R + Sync>(
    items: Vec<T>,
    workers: std::num::NonZeroUsize,
    f: F,
) -> Vec<R> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    use std::sync::{mpsc, Mutex};
    use std::time::Duration;
    #[test]
    fn order_and_borrowing() {
        let suffix = "!".to_owned();
        let words = ["one".to_owned(), "two".to_owned()];
        let out = parallel_map(words.iter().collect(), NonZeroUsize::new(2).unwrap(), |s| {
            format!("{s}{suffix}")
        });
        assert_eq!(out, ["one!", "two!"]);
    }
    #[test]
    fn nonclone_and_empty() {
        struct Item(u8);
        let out = parallel_map(vec![Item(3)], NonZeroUsize::new(4).unwrap(), |x| x.0);
        assert_eq!(out, [3]);
        assert!(parallel_map(Vec::<u8>::new(), NonZeroUsize::new(2).unwrap(), |x| x).is_empty());
    }
    #[test]
    fn work_overlaps_and_is_bounded() {
        let (a_tx, a_rx) = mpsc::channel();
        let (a_done, b_rx) = mpsc::channel();
        let a_rx = Mutex::new(a_rx);
        let b_rx = Mutex::new(b_rx);
        let ids = Mutex::new(std::collections::HashSet::new());
        let output = parallel_map(vec![0, 1], NonZeroUsize::new(2).unwrap(), |i| {
            ids.lock().unwrap().insert(std::thread::current().id());
            if i == 0 {
                a_tx.send(()).unwrap();
                b_rx.lock()
                    .unwrap()
                    .recv_timeout(Duration::from_secs(3))
                    .unwrap();
            } else {
                a_rx.lock()
                    .unwrap()
                    .recv_timeout(Duration::from_secs(3))
                    .unwrap();
                a_done.send(()).unwrap();
            }
            i
        });
        assert_eq!(output, [0, 1]);
        assert_eq!(ids.lock().unwrap().len(), 2);
    }
    #[test]
    #[should_panic]
    fn propagates_panic() {
        parallel_map(vec![1, 2], NonZeroUsize::new(2).unwrap(), |_| -> () {
            panic!("failed")
        });
    }
}
