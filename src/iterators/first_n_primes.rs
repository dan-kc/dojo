// First N Prime Numbers
//
// Learning objectives:
// - Using lazy evaluation with iterators
// - Understanding take() for limiting iterator output
// - Combining filter with infinite ranges
//
// cargo test --bin first_n_primes

/// Create a function that finds the first N prime numbers using iterators.
/// Use lazy evaluation to avoid computing more primes than needed.
pub fn first_n_primes(n: usize) -> Vec<u32> {
    todo!("Use iterator methods with take() for lazy evaluation")
}

/// Helper function to check if a number is prime
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    
    let sqrt_num = (num as f64).sqrt() as u32;
    for i in (3..=sqrt_num).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_n_primes() {
        let primes = first_n_primes(5);
        assert_eq!(primes, vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_first_n_primes_zero() {
        let primes = first_n_primes(0);
        assert_eq!(primes, Vec::<u32>::new());
    }

    #[test]
    fn test_first_n_primes_larger() {
        let primes = first_n_primes(10);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_is_prime_helper() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(9));
        assert!(is_prime(17));
        assert!(!is_prime(15));
    }
}

fn main() {
    println!("Run tests with: cargo test --bin first_n_primes");
}