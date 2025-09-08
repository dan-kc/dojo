// Custom Iterator Practice
//
// Learning objectives:
// - Implementing the Iterator trait
// - Understanding iterator state management  
// - Creating reusable iterator types
// - Iterator combinators and adapters
//
// cargo test --lib iterators::custom_iterator

/// A custom iterator that generates the Fibonacci sequence up to a maximum value.
/// It should stop when the next value would exceed the maximum.
pub struct FibonacciIterator {
    // TODO: Define fields needed to track Fibonacci state
    current: u64,
    next: u64,
    max_value: u64,
}

impl FibonacciIterator {
    /// Create a new Fibonacci iterator that will generate numbers up to (but not exceeding) max_value
    pub fn new(max_value: u64) -> Self {
        todo!("Initialize the iterator with appropriate starting values")
    }
}

impl std::iter::Iterator for FibonacciIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement Fibonacci number generation with max_value limit")
    }
}

/// A custom iterator that yields every nth element from an underlying iterator.
/// For example, EveryNth::new(vec![1,2,3,4,5,6].into_iter(), 2) yields [1, 3, 5]
pub struct EveryNth<I> {
    // TODO: Define fields to track the underlying iterator and step size
    iter: I,
    step: usize,
    current_position: usize,
}

impl<I> EveryNth<I>
where
    I: std::iter::Iterator,
{
    /// Create a new EveryNth iterator that yields every nth element (1-indexed)
    /// n must be >= 1, with n=1 yielding every element, n=2 yielding every other element, etc.
    pub fn new(iter: I, n: usize) -> Self {
        todo!("Initialize with the underlying iterator and step size")
    }
}

impl<I> std::iter::Iterator for EveryNth<I>
where
    I: std::iter::Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement logic to skip elements and yield every nth item")
    }
}

/// A custom iterator that generates a sequence of numbers with a specified step size.
/// Unlike the standard Range, this allows floating-point steps.
pub struct FloatRange {
    // TODO: Define fields for current value, end value, and step size
    current: f64,
    end: f64,
    step: f64,
}

impl FloatRange {
    /// Create a new FloatRange from start to end (exclusive) with the given step size.
    /// Step size must be positive.
    pub fn new(start: f64, end: f64, step: f64) -> Self {
        todo!("Initialize the range iterator")
    }
}

impl std::iter::Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Generate next float in the range, stopping before end")
    }
}

/// A custom iterator that repeats elements from a source iterator infinitely.
/// When the source iterator is exhausted, it restarts from the beginning.
pub struct CycleOwned<T> {
    // TODO: Define fields to store the original data and current position
    items: Vec<T>,
    position: usize,
}

impl<T> CycleOwned<T>
where
    T: Clone,
{
    /// Create a new CycleOwned iterator from a vector of items
    pub fn new(items: Vec<T>) -> Self {
        todo!("Store items and initialize position tracking")
    }
}

impl<T> std::iter::Iterator for CycleOwned<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Cycle through items infinitely, restarting when exhausted")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_iterator_small_max() {
        let fib: Vec<u64> = FibonacciIterator::new(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_fibonacci_iterator_large_max() {
        let fib: Vec<u64> = FibonacciIterator::new(100).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }

    #[test]
    fn test_fibonacci_iterator_zero_max() {
        let fib: Vec<u64> = FibonacciIterator::new(0).collect();
        assert_eq!(fib, vec![0]);
    }

    #[test]
    fn test_fibonacci_iterator_very_small_max() {
        let fib: Vec<u64> = FibonacciIterator::new(1).collect();
        assert_eq!(fib, vec![0, 1, 1]);
    }

    #[test]
    fn test_every_nth_every_second() {
        let source = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result: Vec<i32> = EveryNth::new(source.into_iter(), 2).collect();
        assert_eq!(result, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_every_nth_every_third() {
        let source = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];
        let result: Vec<i32> = EveryNth::new(source.into_iter(), 3).collect();
        assert_eq!(result, vec![10, 40, 70]);
    }

    #[test]
    fn test_every_nth_every_element() {
        let source = vec![1, 2, 3];
        let result: Vec<i32> = EveryNth::new(source.clone().into_iter(), 1).collect();
        assert_eq!(result, source);
    }

    #[test]
    fn test_every_nth_step_larger_than_collection() {
        let source = vec![1, 2, 3];
        let result: Vec<i32> = EveryNth::new(source.into_iter(), 5).collect();
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_float_range_positive_step() {
        let mut range = FloatRange::new(0.0, 3.0, 0.5);
        assert_eq!(range.next(), Some(0.0));
        assert_eq!(range.next(), Some(0.5));
        assert_eq!(range.next(), Some(1.0));
        assert_eq!(range.next(), Some(1.5));
        assert_eq!(range.next(), Some(2.0));
        assert_eq!(range.next(), Some(2.5));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn test_float_range_large_step() {
        let range: Vec<f64> = FloatRange::new(1.0, 5.0, 2.5).collect();
        assert_eq!(range, vec![1.0, 3.5]);
    }

    #[test]
    fn test_float_range_empty() {
        let range: Vec<f64> = FloatRange::new(5.0, 3.0, 1.0).collect();
        assert_eq!(range, Vec::<f64>::new());
    }

    #[test]
    fn test_cycle_owned_basic() {
        let items = vec![1, 2, 3];
        let mut cycle = CycleOwned::new(items);
        
        // First cycle
        assert_eq!(cycle.next(), Some(1));
        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(3));
        
        // Second cycle
        assert_eq!(cycle.next(), Some(1));
        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(3));
        
        // Third cycle begins
        assert_eq!(cycle.next(), Some(1));
    }

    #[test]
    fn test_cycle_owned_single_element() {
        let items = vec!["hello"];
        let mut cycle = CycleOwned::new(items);
        
        for _ in 0..5 {
            assert_eq!(cycle.next(), Some("hello"));
        }
    }

    #[test]
    fn test_cycle_owned_empty() {
        let items: Vec<i32> = vec![];
        let mut cycle = CycleOwned::new(items);
        assert_eq!(cycle.next(), None);
        assert_eq!(cycle.next(), None);
    }

    #[test]
    fn test_cycle_owned_collect_limited() {
        let items = vec!['a', 'b'];
        let result: Vec<char> = CycleOwned::new(items).take(7).collect();
        assert_eq!(result, vec!['a', 'b', 'a', 'b', 'a', 'b', 'a']);
    }
}