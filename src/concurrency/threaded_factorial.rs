// cargo test --lib concurrency::threaded_factorial::

//! Compute factorial on a spawned thread and join it. Return an error for u64 overflow in both debug and release builds; 0! = 1. Return an error if the worker panics. Error text is not prescribed.

/// Create a function that spawns a thread to calculate factorial of a number.
/// The thread should return the result through a JoinHandle.
/// Handle the case where the calculation might panic (overflow).
fn threaded_factorial(n: u64) -> Result<u64, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threaded_factorial() {
        assert_eq!(threaded_factorial(5).unwrap(), 120);
        assert_eq!(threaded_factorial(0).unwrap(), 1);
        assert_eq!(threaded_factorial(1).unwrap(), 1);

        // Test overflow case
        let result = threaded_factorial(25);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test]
    fn overflow_is_error() {
        assert!(threaded_factorial(21).is_err());
        assert_eq!(threaded_factorial(20).unwrap(), 2432902008176640000);
    }
}
