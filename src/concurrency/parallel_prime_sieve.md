# Parallel Prime Sieve

## Solution

```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_prime_sieve(pool: &ThreadPool, n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    
    // Use segmented sieve approach for parallel processing
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    
    let sqrt_n = (n as f64).sqrt() as usize;
    let is_prime = Arc::new(Mutex::new(is_prime));
    let mut handles = Vec::new();
    
    // First, find small primes sequentially (up to sqrt(n))
    {
        let mut sieve = is_prime.lock().unwrap();
        for i in 2..=sqrt_n {
            if sieve[i] {
                for j in ((i * i)..=sqrt_n).step_by(i) {
                    sieve[j] = false;
                }
            }
        }
    }
    
    // Collect small primes for parallel sieving
    let small_primes: Vec<usize> = {
        let sieve = is_prime.lock().unwrap();
        (2..=sqrt_n).filter(|&i| sieve[i]).collect()
    };
    
    // Parallel processing for larger numbers
    let worker_count = pool.worker_count;
    let segment_size = (n - sqrt_n + worker_count - 1) / worker_count;
    
    for worker_id in 0..worker_count {
        let start = sqrt_n + 1 + worker_id * segment_size;
        let end = std::cmp::min(start + segment_size - 1, n);
        
        if start <= end {
            let is_prime_clone = Arc::clone(&is_prime);
            let small_primes_clone = small_primes.clone();
            
            let handle = thread::spawn(move || {
                for &prime in &small_primes_clone {
                    let first_multiple = ((start + prime - 1) / prime) * prime;
                    let first_multiple = std::cmp::max(first_multiple, prime * prime);
                    
                    for multiple in (first_multiple..=end).step_by(prime) {
                        let mut sieve = is_prime_clone.lock().unwrap();
                        sieve[multiple] = false;
                    }
                }
            });
            
            handles.push(handle);
        }
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Collect results
    let sieve = is_prime.lock().unwrap();
    (2..=n).filter(|&i| sieve[i]).collect()
}
```

## Alternative Block-Based Approach

```rust
use std::sync::{Arc, mpsc};
use std::thread;

pub fn parallel_prime_sieve_blocks(pool: &ThreadPool, n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    
    // Sequential sieve for small numbers
    let sqrt_n = (n as f64).sqrt() as usize;
    let mut small_primes = Vec::new();
    let mut is_prime_small = vec![true; sqrt_n + 1];
    
    for i in 2..=sqrt_n {
        if is_prime_small[i] {
            small_primes.push(i);
            for j in ((i * i)..=sqrt_n).step_by(i) {
                is_prime_small[j] = false;
            }
        }
    }
    
    // Parallel processing for larger ranges
    let block_size = 10000;
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    
    for block_start in ((sqrt_n + 1)..=n).step_by(block_size) {
        let block_end = std::cmp::min(block_start + block_size - 1, n);
        let small_primes_clone = small_primes.clone();
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            let mut block_primes = Vec::new();
            let block_len = block_end - block_start + 1;
            let mut is_prime = vec![true; block_len];
            
            // Apply each small prime to this block
            for &prime in &small_primes_clone {
                let first_multiple = ((block_start + prime - 1) / prime) * prime;
                let first_multiple = std::cmp::max(first_multiple, prime * prime);
                
                for multiple in (first_multiple..=block_end).step_by(prime) {
                    let index = multiple - block_start;
                    is_prime[index] = false;
                }
            }
            
            // Collect primes in this block
            for (i, &prime_flag) in is_prime.iter().enumerate() {
                if prime_flag {
                    block_primes.push(block_start + i);
                }
            }
            
            tx_clone.send(block_primes).unwrap();
        });
        
        handles.push(handle);
    }
    
    drop(tx);
    
    // Collect results from all blocks
    let mut all_primes = small_primes;
    for block_primes in rx {
        all_primes.extend(block_primes);
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    all_primes.sort();
    all_primes
}
```

## Explanation

The parallel prime sieve uses a segmented approach that combines sequential and parallel processing:

1. **Sequential Phase**: First finds all primes up to √n using a standard sequential sieve, as these are needed to sieve the larger numbers.

2. **Work Distribution**: Divides the remaining range (√n to n) into segments that can be processed independently by different threads.

3. **Parallel Sieving**: Each worker thread applies all the small primes to its segment, marking multiples as composite.

4. **Synchronization**: Uses `Arc<Mutex<>>` to protect the shared sieve array, or alternatively processes separate blocks and combines results.

5. **Result Collection**: After all threads complete, collects all numbers still marked as prime.

The block-based alternative avoids lock contention by having each thread work on a separate memory region and then combining results. This approach often performs better for larger ranges due to reduced synchronization overhead.

Both implementations maintain correctness while providing significant speedup for large ranges through parallel processing of independent segments.