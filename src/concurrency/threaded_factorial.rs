// Threaded Factorial Calculation Practice
//
// Learning Objectives:
// - Create threads for computational tasks
// - Handle thread panics and Results
// - Work with error propagation from threads
//
// cargo test --bin threaded_factorial

/// Create a function that spawns a thread to calculate factorial of a number.
/// The thread should return the result through a JoinHandle.
/// Handle the case where the calculation might panic (overflow).
fn threaded_factorial(n: u64) -> Result<u64, String> {
    todo!("Implement threaded factorial calculation")
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