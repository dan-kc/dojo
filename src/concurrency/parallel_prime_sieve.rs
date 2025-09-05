// Parallel Prime Sieve Practice
//
// Learning Objectives:
// - Implement parallel algorithms for mathematical computation
// - Use threading for CPU-intensive tasks
// - Coordinate parallel workers for complex algorithms
// - Optimize parallel processing for prime number generation
//
// cargo test --bin parallel_prime_sieve

/// Basic ThreadPool stub for parallel prime sieve implementation
pub struct ThreadPool {
    worker_count: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        Self { worker_count: size }
    }
}

/// Calculate prime numbers up to n using parallel processing
pub fn parallel_prime_sieve(pool: &ThreadPool, n: usize) -> Vec<usize> {
    todo!("Implement parallel prime sieve")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_prime_sieve_basic() {
        let pool = ThreadPool::new(4);
        let primes = parallel_prime_sieve(&pool, 30);
        
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert_eq!(primes, expected);
    }

    #[test]
    fn test_small_numbers() {
        let pool = ThreadPool::new(2);
        
        let primes_10 = parallel_prime_sieve(&pool, 10);
        assert_eq!(primes_10, vec![2, 3, 5, 7]);
        
        let primes_20 = parallel_prime_sieve(&pool, 20);
        assert_eq!(primes_20, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_edge_cases() {
        let pool = ThreadPool::new(3);
        
        // Test n = 2 (smallest prime)
        let primes_2 = parallel_prime_sieve(&pool, 2);
        assert_eq!(primes_2, vec![2]);
        
        // Test n = 1 (no primes)
        let primes_1 = parallel_prime_sieve(&pool, 1);
        assert!(primes_1.is_empty());
        
        // Test n = 0 (no primes)
        let primes_0 = parallel_prime_sieve(&pool, 0);
        assert!(primes_0.is_empty());
    }

    #[test]
    fn test_larger_range() {
        let pool = ThreadPool::new(6);
        let primes = parallel_prime_sieve(&pool, 100);
        
        // First few primes should be correct
        assert!(primes.starts_with(&[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]));
        
        // Should contain known primes
        assert!(primes.contains(&97)); // 97 is prime
        assert!(!primes.contains(&98)); // 98 is not prime (2 * 49)
        assert!(!primes.contains(&99)); // 99 is not prime (3 * 33)
        
        // Count should match expected number of primes up to 100
        assert_eq!(primes.len(), 25); // There are 25 primes up to 100
    }

    #[test]
    fn test_prime_properties() {
        let pool = ThreadPool::new(4);
        let primes = parallel_prime_sieve(&pool, 50);
        
        // All results should be prime numbers
        for &p in &primes {
            assert!(is_prime(p), "{} is not prime", p);
        }
        
        // Should be in ascending order
        for i in 1..primes.len() {
            assert!(primes[i - 1] < primes[i], "Primes not in order");
        }
    }

    #[test]
    fn test_comparison_with_sequential() {
        let pool = ThreadPool::new(4);
        let parallel_primes = parallel_prime_sieve(&pool, 200);
        let sequential_primes = sequential_sieve(200);
        
        assert_eq!(parallel_primes, sequential_primes);
    }

    #[test]
    fn test_different_thread_counts() {
        // Test that different thread counts produce same results
        let n = 150;
        
        let pool1 = ThreadPool::new(1);
        let primes1 = parallel_prime_sieve(&pool1, n);
        
        let pool2 = ThreadPool::new(4);
        let primes2 = parallel_prime_sieve(&pool2, n);
        
        let pool3 = ThreadPool::new(8);
        let primes3 = parallel_prime_sieve(&pool3, n);
        
        assert_eq!(primes1, primes2);
        assert_eq!(primes2, primes3);
    }

    // Helper function to verify primality
    fn is_prime(n: usize) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        
        let sqrt_n = (n as f64).sqrt() as usize;
        for i in (3..=sqrt_n).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    // Sequential sieve for comparison
    fn sequential_sieve(n: usize) -> Vec<usize> {
        if n < 2 {
            return Vec::new();
        }
        
        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        if n >= 1 {
            is_prime[1] = false;
        }
        
        for i in 2..=((n as f64).sqrt() as usize) {
            if is_prime[i] {
                for j in ((i * i)..=n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }
        
        (2..=n).filter(|&i| is_prime[i]).collect()
    }
}