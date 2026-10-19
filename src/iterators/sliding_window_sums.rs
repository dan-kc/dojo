// cargo test --lib iterators::sliding_window_sums::

//! Return the i64 sum of each contiguous window of width items, in order.
//! Width is nonzero by type. A wider-than-input window yields no results.
//! Use O(n) time and O(1) auxiliary space besides output; avoid summing each window from scratch.

pub fn sliding_window_sums(numbers: &[i32], width: std::num::NonZeroUsize) -> Vec<i64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    #[test]
    fn overlapping() {
        assert_eq!(
            sliding_window_sums(&[1, 2, 3, 4, 5], NonZeroUsize::new(3).unwrap()),
            [6, 9, 12]
        );
    }
    #[test]
    fn wide_values() {
        assert_eq!(
            sliding_window_sums(&[i32::MAX, i32::MAX, -1], NonZeroUsize::new(2).unwrap()),
            [4294967294, 2147483646]
        );
    }
    #[test]
    fn boundaries() {
        assert!(sliding_window_sums(&[], NonZeroUsize::new(1).unwrap()).is_empty());
        assert!(sliding_window_sums(&[1], NonZeroUsize::new(2).unwrap()).is_empty());
        assert_eq!(
            sliding_window_sums(&[3, -2], NonZeroUsize::new(1).unwrap()),
            [3, -2]
        );
    }
}
