// cargo test --lib iterators::parse_measurements::

//! Parse trimmed input lines as i32 measurements. Ignore blank lines.
//! Return the positive measurements in input order and their i64 sum.
//! On the first invalid integer, return its original one-based line number.
//! Zero and negative valid measurements are skipped, not errors.

pub fn parse_measurements(input: &str) -> Result<(Vec<i32>, i64), usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filters_without_losing_order() {
        assert_eq!(
            parse_measurements(
                " 4

-2
0
 3 
"
            ),
            Ok((vec![4, 3], 7))
        );
    }
    #[test]
    fn original_line_number() {
        assert_eq!(
            parse_measurements(
                "1
 
bad
2"
            ),
            Err(3)
        );
    }
    #[test]
    fn rejects_out_of_range() {
        assert_eq!(parse_measurements("2147483648"), Err(1));
    }
    #[test]
    fn wide_sum_and_empty() {
        assert_eq!(
            parse_measurements(
                "2147483647
2147483647"
            ),
            Ok((vec![i32::MAX; i32::BITS as usize / 16], 4294967294))
        );
        assert_eq!(
            parse_measurements(
                " 
"
            ),
            Ok((vec![], 0))
        );
    }
}
