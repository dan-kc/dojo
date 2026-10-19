// cargo test --lib iterators::custom_iterator::

//! Implement a lazy iterator over Fibonacci values 0,1,1,2,... no greater than max_value.
//! Stop cleanly on u64 overflow. Once exhausted it must always return None.
//! Keep constant-size state; do not precompute the sequence.

pub struct FibonacciIterator {
    current: Option<u64>,
    next: Option<u64>,
    max_value: u64,
}
impl FibonacciIterator {
    pub fn new(max_value: u64) -> Self {
        todo!()
    }
}
impl Iterator for FibonacciIterator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sequence() {
        assert_eq!(
            FibonacciIterator::new(10).collect::<Vec<_>>(),
            [0, 1, 1, 2, 3, 5, 8]
        );
    }
    #[test]
    fn boundaries() {
        assert_eq!(FibonacciIterator::new(0).collect::<Vec<_>>(), [0]);
        assert_eq!(FibonacciIterator::new(1).collect::<Vec<_>>(), [0, 1, 1]);
    }
    #[test]
    fn overflow_exhausts() {
        let mut fib = FibonacciIterator::new(u64::MAX);
        let values: Vec<_> = fib.by_ref().collect();
        assert_eq!(values.len(), 94);
        assert_eq!(values.last(), Some(&12200160415121876738));
        assert_eq!(fib.next(), None);
        assert_eq!(fib.next(), None);
    }
}
