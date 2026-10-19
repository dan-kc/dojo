// cargo test --lib iterators::sum_squares_evens::

//! Return the sum of the squares of even i32 values as i128. Borrow the input and use no intermediate collection.
//! Empty input returns zero. Choose any clear linear-time implementation; iterator syntax alone is not a performance guarantee.

pub fn sum_squares_evens_iterator(numbers: &[i32]) -> i128 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_values() {
        assert_eq!(sum_squares_evens_iterator(&[1, 2, 3, 4, -6]), 56);
    }
    #[test]
    fn wide_accumulation() {
        assert_eq!(
            sum_squares_evens_iterator(&[i32::MIN; i32::BITS as usize / 8]),
            18446744073709551616
        );
    }
    #[test]
    fn empty() {
        assert_eq!(sum_squares_evens_iterator(&[]), 0);
    }
}
