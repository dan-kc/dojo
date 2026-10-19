// cargo test --lib concurrency::parallel_reduce::

//! Reduce owned items using at most workers threads and join every worker before returning.
//! The caller provides an associative operation with a true two-sided identity. Preserve item order;
//! commutativity is not required. Regrouping is allowed. Empty input returns identity.
//! Borrowed closure captures and non-Clone items must work. Propagate worker panics after joining all workers.

pub fn parallel_reduce<T: Send, F: Fn(T, T) -> T + Sync>(
    items: Vec<T>,
    workers: std::num::NonZeroUsize,
    identity: T,
    reduce: F,
) -> T {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    #[test]
    fn sum_and_empty() {
        let w = NonZeroUsize::new(3).unwrap();
        assert_eq!(parallel_reduce(vec![1, 2, 3, 4], w, 0, |a, b| a + b), 10);
        assert_eq!(parallel_reduce(Vec::<i32>::new(), w, 0, |a, b| a + b), 0);
    }
    #[test]
    fn noncommutative_nonclone_values() {
        struct Text(String);
        let out = parallel_reduce(
            vec![Text("a".into()), Text("b".into()), Text("c".into())],
            NonZeroUsize::new(2).unwrap(),
            Text(String::new()),
            |a, b| Text(a.0 + &b.0),
        );
        assert_eq!(out.0, "abc");
    }
    #[test]
    #[should_panic]
    fn panic_propagates() {
        parallel_reduce(
            vec![1, 2, 3, 4],
            NonZeroUsize::new(2).unwrap(),
            0,
            |_, _| panic!("bad"),
        );
    }
}
