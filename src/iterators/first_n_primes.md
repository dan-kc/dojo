# iterators/first_n_primes

Reference implementation. Read after attempting the exercise.

```rust
pub fn first_n_primes(n: usize) -> Vec<u32> {
    (2u32..).filter(|&num| is_prime(num)).take(n).collect()
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
```
