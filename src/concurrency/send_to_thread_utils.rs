// Send to Thread Utilities Practice
//
// Learning Objectives:
// - Work with Send trait bounds in function signatures
// - Practice moving data between threads
// - Use generic functions with Send constraints
//
// Run with: cargo test --bin send_to_thread_utils

use std::thread;

pub fn send_to_thread<T: Send + 'static, F: FnOnce(T) -> T + Send + 'static>(
    value: T,
    processor: F,
) -> T {
    todo!("Implement send_to_thread")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_to_thread() {
        let result = send_to_thread(vec![1, 2, 3, 4, 5], |mut v| {
            v.iter_mut().for_each(|x| *x *= 2);
            v
        });

        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}